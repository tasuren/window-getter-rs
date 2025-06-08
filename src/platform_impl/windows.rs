pub use bounds::PlatformBounds;
pub use error::PlatformError;
pub use window::PlatformWindow;
use windows::{Win32::Foundation::LPARAM, core::BOOL};

use crate::{Error, Window};

unsafe extern "system" fn enum_windows_callback(
    hwnd: windows::Win32::Foundation::HWND,
    lparam: LPARAM,
) -> BOOL {
    let windows = unsafe { &mut *(lparam.0 as *mut Vec<Window>) };
    windows.push(Window(PlatformWindow(hwnd)));

    BOOL::from(true)
}

/// Retrieves a list of all windows on the screen.
pub fn get_windows() -> Result<Vec<Window>, Error> {
    let mut windows = Vec::new();

    // SAFETY: `Vec` should not be used during enumeration because it is used by mutable reference.
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::EnumWindows(
            Some(enum_windows_callback),
            LPARAM(&mut windows as *const Vec<Window> as _),
        )?
    };

    Ok(windows)
}

mod window {
    use std::mem::MaybeUninit;

    use windows::Win32::{
        Foundation::{self, HWND},
        System::Threading,
        UI::WindowsAndMessaging,
    };

    use super::PlatformError;
    use crate::platform_impl::windows::PlatformBounds;

    /// Represents a window in the Windows platform.
    pub struct PlatformWindow(pub(crate) HWND);

    unsafe impl Send for PlatformWindow {}
    unsafe impl Sync for PlatformWindow {}

    impl PlatformWindow {
        /// Creates a new [`PlatformWindow`] from a raw [`HWND`](windows::Win32::Foundation::HWND).
        ///
        /// # Safety
        /// You must ensure that the `hwnd` is a valid window handle.
        pub unsafe fn new(hwnd: HWND) -> Self {
            Self(hwnd)
        }

        /// Returns the raw handle to the window.
        pub fn hwnd(&self) -> HWND {
            self.0
        }

        /// Returns the title of the window.
        pub fn title(&self) -> Result<Option<String>, PlatformError> {
            let mut buffer = [0u16; 256];
            let length = unsafe { WindowsAndMessaging::GetWindowTextW(self.0, &mut buffer) };

            if length == 0 {
                let raw = windows::core::Error::from_win32();

                return match raw.code() {
                    // If the length is 0 and error is success,
                    // it means the window has no title.
                    Foundation::S_OK => Ok(None),
                    _ => Err(raw),
                };
            }

            Ok(Some(String::from_utf16_lossy(&buffer[..length as usize])))
        }

        /// Returns the bounds of the window.
        pub fn bounds(&self) -> Result<PlatformBounds, PlatformError> {
            let mut value = MaybeUninit::uninit();

            let rect = unsafe {
                WindowsAndMessaging::GetWindowRect(self.0, value.as_mut_ptr())?;
                value.assume_init()
            };

            Ok(PlatformBounds(rect))
        }

        /// Returns the process ID of the owner of this window.
        pub fn owner_pid(&self) -> Result<i32, PlatformError> {
            let mut pid = 0;
            let thread =
                unsafe { WindowsAndMessaging::GetWindowThreadProcessId(self.0, Some(&mut pid)) };

            if thread == 0 {
                Err(windows::core::Error::from_win32())
            } else {
                Ok(pid as _)
            }
        }

        /// Returns the handle to the process that owns this window.
        pub fn owner_process_handle(
            &self,
        ) -> Result<windows::Win32::Foundation::HANDLE, PlatformError> {
            let pid = self.owner_pid()?;
            let process_handle = unsafe {
                Threading::OpenProcess(
                    Threading::PROCESS_QUERY_INFORMATION | Threading::PROCESS_VM_READ,
                    false,
                    pid as _,
                )?
            };

            Ok(process_handle)
        }

        /// Returns the file name of the process that owns this window.
        /// This will return the name of the executable file.
        pub fn owner_name(&self) -> Result<String, PlatformError> {
            let process_handle = self.owner_process_handle()?;

            let mut buffer = [0u16; 256];
            let length = unsafe {
                windows::Win32::System::ProcessStatus::GetModuleBaseNameW(
                    process_handle,
                    None,
                    &mut buffer,
                )
            };

            if length == 0 {
                return Err(windows::core::Error::from_win32());
            }

            Ok(String::from_utf16_lossy(&buffer[..length as usize]))
        }
    }
}

mod bounds {
    use windows::Win32::Foundation::RECT;

    /// Represents the bounds of a window in the Windows platform.
    pub struct PlatformBounds(pub(crate) RECT);

    impl PlatformBounds {
        /// Creates a new [`PlatformBounds`] from a raw [`RECT`](windows::Win32::Foundation::RECT).
        pub fn new(rect: RECT) -> Self {
            Self(rect)
        }

        /// Returns the raw [`RECT`](windows::Win32::Foundation::RECT) structure.
        pub fn sys(&self) -> &RECT {
            &self.0
        }

        /// Returns the x-coordinate of the bounds.
        pub fn x(&self) -> i32 {
            self.0.left
        }

        /// Returns the y-coordinate of the bounds.
        pub fn y(&self) -> i32 {
            self.0.top
        }

        /// Returns the width of the bounds.
        /// The width is calculated as `right - left`
        /// by using [`RECT`](windows::Win32::Foundation::RECT).
        pub fn width(&self) -> i32 {
            self.0.right - self.0.left
        }

        /// Returns the height of the bounds.
        /// The width is calculated as `bottom - top`
        /// by using [`RECT`](windows::Win32::Foundation::RECT).
        pub fn height(&self) -> i32 {
            self.0.bottom - self.0.top
        }

        /// Returns the left coordinate of the bounds.
        pub const fn left(&self) -> i32 {
            self.0.left
        }

        /// Returns the top coordinate of the bounds.
        pub const fn top(&self) -> i32 {
            self.0.top
        }

        /// Returns the right coordinate of the bounds.
        pub const fn right(&self) -> i32 {
            self.0.right
        }

        /// Returns the bottom coordinate of the bounds.
        pub const fn bottom(&self) -> i32 {
            self.0.bottom
        }
    }
}

mod error {
    pub use windows::core::Error as PlatformError;

    impl From<windows::core::Error> for crate::Error {
        fn from(error: windows::core::Error) -> Self {
            if error.code() == windows::Win32::Foundation::E_ACCESSDENIED {
                Self::PermissionDenied(error)
            } else {
                Self::PlatformSpecificError(error)
            }
        }
    }
}
