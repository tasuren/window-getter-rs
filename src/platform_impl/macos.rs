use objc2_core_foundation::{CFArray, CFDictionary, CFRetained, CFString, CFType};
use objc2_core_graphics::{CGWindowListCopyWindowInfo, CGWindowListOption, kCGNullWindowID};

use crate::{Error, Window};

pub use bounds::PlatformBounds;
pub use error::PlatformError;
pub use window::PlatformWindow;
pub use window_info::WindowInfo;

/// Retrieves a list of all open windows on the system.
pub fn get_windows() -> Result<Vec<Window>, Error> {
    let list: CFRetained<CFArray<CFDictionary<CFString, CFType>>> = unsafe {
        let list = CGWindowListCopyWindowInfo(CGWindowListOption::all(), kCGNullWindowID);
        let Some(list) = list else {
            return Err(Error::NoWindowEnvironment);
        };

        CFRetained::cast_unchecked(list)
    };

    let windows = list
        .iter()
        .map(|dict| Window(PlatformWindow(WindowInfo(dict))))
        .collect();

    Ok(windows)
}

mod bounds {
    use objc2_core_foundation::CGRect;

    /// A wrapper around a `CGRect` that represents the bounds of a window.
    pub struct PlatformBounds(pub(crate) CGRect);

    impl PlatformBounds {
        pub const fn new(rect: CGRect) -> Self {
            Self(rect)
        }

        pub const fn cg_rect(&self) -> CGRect {
            self.0
        }

        pub const fn x(&self) -> f64 {
            self.0.origin.x
        }

        pub const fn y(&self) -> f64 {
            self.0.origin.y
        }

        pub const fn width(&self) -> f64 {
            self.0.size.width
        }

        pub const fn height(&self) -> f64 {
            self.0.size.height
        }
    }
}

mod window {
    use std::mem::MaybeUninit;

    use objc2_core_foundation::CGRect;
    use objc2_core_graphics::CGRectMakeWithDictionaryRepresentation;

    use crate::platform_impl::{PlatformBounds, macos::PlatformError};

    use super::WindowInfo;

    /// A wrapper around a window's information [WindowInfo](super::WindowInfo).
    pub struct PlatformWindow(pub(crate) WindowInfo);

    impl PlatformWindow {
        /// Creates a new [`PlatformWindow`] from a [`WindowInfo`](super::WindowInfo).
        pub const fn new(window_info: WindowInfo) -> Self {
            Self(window_info)
        }

        /// Returns the underlying [`WindowInfo`](super::WindowInfo) object.
        pub const fn window_info(&self) -> &WindowInfo {
            &self.0
        }

        /// Returns the window's title.
        pub fn title(&self) -> Option<String> {
            self.0.name().map(|name| name.to_string())
        }

        /// Returns the bounds of the window as a [`PlatformBounds`](super::PlatformError).
        pub fn bounds(&self) -> Result<PlatformBounds, PlatformError> {
            let bounds = self.0.bounds();
            let mut rect = MaybeUninit::<CGRect>::uninit();

            unsafe {
                let result =
                    CGRectMakeWithDictionaryRepresentation(Some(&bounds), rect.as_mut_ptr());

                if result {
                    Ok(PlatformBounds(rect.assume_init()))
                } else {
                    Err(PlatformError::InvalidWindowBounds)
                }
            }
        }

        /// Returns the process ID of the window's owner.
        pub fn owner_pid(&self) -> i32 {
            self.0
                .owner_pid()
                .as_i32()
                .expect("invalid owner PID value")
        }

        /// Returns the name of the process that owns the window.
        pub fn owner_name(&self) -> Option<String> {
            self.0.owner_name().map(|name| name.to_string())
        }
    }
}

mod window_info {
    use objc2_core_foundation::{CFBoolean, CFDictionary, CFNumber, CFRetained, CFString, CFType};

    macro_rules! impl_window_info_getters {
        ($(($name:ident, $return_type:ty, $key:ident)),*) => {
            $(
                pub fn $name(&self) -> CFRetained<$return_type> {
                    let object = self
                        .0
                        .get(unsafe { objc2_core_graphics::$key })
                        .expect(concat!("`", stringify!($key), "` should always be present"));

                    const EXPECT: &str = concat!(
                        "Expected a value `",
                        stringify!($return_type),
                        "` for the key `",
                        stringify!($key),
                        "`"
                    );

                    CFRetained::downcast(object).expect(EXPECT)
                }
            )*
        };
    }

    macro_rules! impl_window_info_optional_getters {
        ($(($name:ident, $return_type:ty, $key:ident)),*) => {
            $(
                pub fn $name(&self) -> Option<CFRetained<$return_type>> {
                    let object = self
                        .0
                        .get(unsafe { objc2_core_graphics::$key })?;

                    const EXPECT: &str = concat!(
                        "Expected a value `",
                        stringify!($return_type),
                        "` for the key `",
                        stringify!($key),
                        "`"
                    );

                    Some(CFRetained::downcast(object).expect(EXPECT))
                }
            )*
        };
    }

    /// The wrapper for a window's dictionary representation.
    ///
    /// # See also
    /// This struct represents a window's information and supports following values:
    /// - [Required Window List Keys](https://developer.apple.com/documentation/coregraphics/required-window-list-keys?language=objc)
    /// - [Optional Window List Keys](https://developer.apple.com/documentation/coregraphics/optional-window-list-keys?language=objc)
    pub struct WindowInfo(pub(super) CFRetained<CFDictionary<CFString, CFType>>);

    unsafe impl Send for WindowInfo {}
    unsafe impl Sync for WindowInfo {}

    impl WindowInfo {
        /// Creates a new `WindowInfo` from a retained dictionary.
        ///
        /// # Safety
        /// You must ensure that the dictionary is a valid representation of a window's information.
        /// See also the corresponding documentation [Required Window List Keys][required] and
        /// [Optional Window List Keys][optional] about a valid representation.
        ///
        /// [required]: <https://developer.apple.com/documentation/coregraphics/required-window-list-keys?language=objc>
        /// [optional]: <https://developer.apple.com/documentation/coregraphics/optional-window-list-keys?language=objc>
        pub unsafe fn new(dict: CFRetained<CFDictionary<CFString, CFType>>) -> Self {
            Self(dict)
        }

        pub const fn sys(&self) -> &CFRetained<CFDictionary<CFString, CFType>> {
            &self.0
        }

        impl_window_info_getters!(
            (number, CFNumber, kCGWindowNumber),
            (store_type, CFNumber, kCGWindowStoreType),
            (layer, CFNumber, kCGWindowLayer),
            (bounds, CFDictionary, kCGWindowBounds),
            (sharing_state, CFNumber, kCGWindowSharingState),
            (alpha, CFNumber, kCGWindowAlpha),
            (owner_pid, CFNumber, kCGWindowOwnerPID),
            (memory_usage, CFNumber, kCGWindowMemoryUsage)
        );

        impl_window_info_optional_getters!(
            (owner_name, CFString, kCGWindowOwnerName),
            (name, CFString, kCGWindowName),
            (is_on_screen, CFBoolean, kCGWindowIsOnscreen),
            (
                backing_location_video_memory,
                CFBoolean,
                kCGWindowBackingLocationVideoMemory
            )
        );
    }
}

mod error {
    /// Low-level errors that can occur when interacting with the platform-specific API.
    #[derive(Debug, thiserror::Error)]
    pub enum PlatformError {
        /// Represents a situation when the window bounds cannot be used.
        #[error("Failed to make window `CGRect` from dictionary representation.")]
        InvalidWindowBounds,
    }

    impl From<PlatformError> for crate::Error {
        fn from(error: PlatformError) -> Self {
            Self::PlatformSpecificError(error)
        }
    }
}
