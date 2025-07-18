use windows::{
    Win32::{Foundation::LPARAM, UI::WindowsAndMessaging::IsWindow},
    core::BOOL,
};

use crate::{Error, Window, platform_impl::windows::window::WindowsWindow};

pub type PlatformBounds = windows::Win32::Foundation::RECT;
pub type PlatformError = error::WindowsError;
pub type PlatformWindow = window::WindowsWindow;
pub type PlatformWindowId = window_id::WindowsWindowId;

/// Retrieves a window by its platform-specific identifier ([`HWND`](windows::Win32::Foundation::HWND)).
pub fn get_window(id: PlatformWindowId) -> Option<Window> {
    let hwnd = id;

    if hwnd.is_invalid() || !unsafe { IsWindow(Some(hwnd)) }.as_bool() {
        None
    } else {
        Some(Window(unsafe { PlatformWindow::new_unchecked(hwnd) }))
    }
}

unsafe extern "system" fn enum_windows_callback(
    hwnd: windows::Win32::Foundation::HWND,
    lparam: LPARAM,
) -> BOOL {
    let windows = unsafe { &mut *(lparam.0 as *mut Vec<Window>) };
    windows.push(Window(unsafe { WindowsWindow::new_unchecked(hwnd) }));

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
        Foundation::{self, HWND, RECT},
        System::Threading,
        UI::WindowsAndMessaging,
    };

    use super::PlatformError;

    /// Represents a window in the Windows platform.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WindowsWindow(pub(crate) HWND);

    unsafe impl Send for WindowsWindow {}
    unsafe impl Sync for WindowsWindow {}

    impl WindowsWindow {
        /// Creates a new `PlatformWindow` from a raw [`HWND`].
        ///
        /// # Safety
        /// You must ensure that the `hwnd` is a valid window handle.
        pub unsafe fn new_unchecked(hwnd: HWND) -> Self {
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
        pub fn bounds(&self) -> Result<RECT, PlatformError> {
            let mut value = MaybeUninit::uninit();

            let rect = unsafe {
                WindowsAndMessaging::GetWindowRect(self.0, value.as_mut_ptr())?;
                value.assume_init()
            };

            Ok(rect)
        }

        /// Returns the process ID of the owner of this window.
        pub fn owner_pid(&self) -> Result<u32, PlatformError> {
            let mut pid = 0;
            let thread =
                unsafe { WindowsAndMessaging::GetWindowThreadProcessId(self.0, Some(&mut pid)) };

            if thread == 0 {
                Err(windows::core::Error::from_win32())
            } else {
                Ok(pid)
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
                    pid,
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

        /// Checks if the window is foreground.
        pub fn is_foreground(&self) -> bool {
            self.0 == unsafe { WindowsAndMessaging::GetForegroundWindow() }
        }
    }
}

pub mod error {
    pub type WindowsError = windows::core::Error;

    impl From<WindowsError> for crate::Error {
        fn from(error: WindowsError) -> Self {
            if error.code() == windows::Win32::Foundation::E_ACCESSDENIED {
                Self::PermissionDenied(error)
            } else {
                Self::PlatformSpecificError(error)
            }
        }
    }
}

mod window_id {
    pub type WindowsWindowId = windows::Win32::Foundation::HWND;

    use crate::WindowId;

    impl From<WindowId> for WindowsWindowId {
        fn from(id: WindowId) -> Self {
            id.0
        }
    }
}
