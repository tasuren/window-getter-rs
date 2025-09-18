use objc2_core_foundation::{CFArray, CFDictionary, CFRetained, CFString, CFType};
use objc2_core_graphics::{CGWindowListCopyWindowInfo, CGWindowListOption, kCGNullWindowID};

use crate::{Error, Window};

pub use window_info::WindowInfo;

pub type PlatformBounds = objc2_core_foundation::CGRect;
pub type PlatformError = error::MacOSError;
pub type PlatformWindow = window::MacOSWindow;
pub type PlatformWindowId = objc2_core_graphics::CGWindowID;

/// Retrieves a window by its unique identifier.
pub fn get_window(id: PlatformWindowId) -> Result<Option<Window>, Error> {
    let list: CFRetained<CFArray<CFDictionary<CFString, CFType>>> = unsafe {
        let list = CGWindowListCopyWindowInfo(CGWindowListOption::all(), id);
        let Some(list) = list else {
            return Err(Error::NoWindowEnvironment);
        };

        CFRetained::cast_unchecked(list)
    };

    for dict in list.iter() {
        let window = PlatformWindow::new(WindowInfo::new(dict));
        if window.id() == id {
            return Ok(Some(Window(window)));
        }
    }

    Ok(None)
}

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
        .map(|dict| Window(PlatformWindow::new(WindowInfo::new(dict))))
        .collect();

    Ok(windows)
}

pub mod window {
    use std::mem::MaybeUninit;

    use objc2_core_foundation::CGRect;
    use objc2_core_graphics::CGRectMakeWithDictionaryRepresentation;

    use crate::{Bounds, platform_impl::macos::PlatformError};

    use super::WindowInfo;

    /// A wrapper around a window's information [`WindowInfo`].
    #[derive(Clone, Debug)]
    pub struct MacOSWindow(pub(crate) WindowInfo);

    impl MacOSWindow {
        /// Creates a new [`MacOSWindow`] from a [`WindowInfo`].
        pub fn new(window_info: WindowInfo) -> Self {
            Self(window_info)
        }

        /// Returns the underlying [`WindowInfo`] object.
        pub fn window_info(&self) -> &WindowInfo {
            &self.0
        }

        /// Returns the window unique identifier.
        /// This is the window number.
        pub fn id(&self) -> u32 {
            self.0
                .number()
                .as_i64()
                .expect("invalid window number value") as _
        }

        /// Returns the window's title.
        pub fn title(&self) -> Option<String> {
            self.id();
            self.0.name().map(|name| name.to_string())
        }

        /// Returns the bounds of the window as a [`Bounds`].
        pub fn bounds(&self) -> Result<Bounds, PlatformError> {
            let bounds = self.0.bounds();
            let mut rect = MaybeUninit::<CGRect>::uninit();

            unsafe {
                let result =
                    CGRectMakeWithDictionaryRepresentation(Some(&bounds), rect.as_mut_ptr());

                if result {
                    Ok(rect.assume_init().into())
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

pub mod window_info {
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
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct WindowInfo(CFRetained<CFDictionary<CFString, CFType>>);

    unsafe impl Send for WindowInfo {}
    unsafe impl Sync for WindowInfo {}

    impl WindowInfo {
        /// Creates a new `WindowInfo` from a retained dictionary.
        ///
        /// # Panics
        /// You must ensure that the dictionary is a valid representation of a window's information.
        /// See also the corresponding documentation [Required Window List Keys][required] and
        /// [Optional Window List Keys][optional] about a valid representation.
        ///
        /// [required]: <https://developer.apple.com/documentation/coregraphics/required-window-list-keys?language=objc>
        /// [optional]: <https://developer.apple.com/documentation/coregraphics/optional-window-list-keys?language=objc>
        pub fn new(dict: CFRetained<CFDictionary<CFString, CFType>>) -> Self {
            Self(dict)
        }

        pub fn sys(&self) -> &CFRetained<CFDictionary<CFString, CFType>> {
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

/// Module for handling screen capture permissions on macOS.
/// Most informations of the window cannot be retrieved without this permission. e.g. title
pub mod permission {
    /// Requests screen capture access permission from the user.
    pub fn request_screen_capture_access() -> bool {
        unsafe { objc2_core_graphics::CGRequestScreenCaptureAccess() }
    }

    /// Checks if the application has screen capture access permission.
    pub fn has_screen_capture_access() -> bool {
        unsafe { objc2_core_graphics::CGPreflightScreenCaptureAccess() }
    }
}

pub mod error {
    /// Low-level errors that can occur when interacting with the platform-specific API.
    #[derive(Debug, thiserror::Error)]
    pub enum MacOSError {
        /// Represents a situation when the window bounds cannot be used.
        #[error("Failed to make window `CGRect` from dictionary representation.")]
        InvalidWindowBounds,
    }

    impl From<MacOSError> for crate::Error {
        fn from(error: MacOSError) -> Self {
            Self::PlatformSpecificError(error)
        }
    }
}
