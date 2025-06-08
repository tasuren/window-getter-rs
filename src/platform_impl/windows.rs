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

    use windows::Win32::{Foundation::HWND, System::Threading, UI::WindowsAndMessaging};

    use super::PlatformError;
    use crate::platform_impl::windows::PlatformBounds;

    pub struct PlatformWindow(pub(crate) HWND);

    impl PlatformWindow {
        /// Creates a new `PlatformWindow` from a raw `HWND`.
        ///
        /// # Safety
        /// You must ensure that the `hwnd` is a valid window handle.
        pub unsafe fn new(hwnd: HWND) -> Self {
            Self(hwnd)
        }

        pub fn sys(&self) -> HWND {
            self.0
        }

        pub fn title(&self) -> Result<Option<String>, PlatformError> {
            let mut buffer = [0u16; 256];
            let length = unsafe { WindowsAndMessaging::GetWindowTextW(self.0, &mut buffer) };

            if length == 0 {
                let raw = windows::core::Error::from_win32();
                if raw.code() == windows::Win32::Foundation::S_OK {
                    // If the length is 0 and error is success,
                    // it means the window has no title.
                    return Ok(None);
                }

                return Err(raw);
            }

            Ok(Some(String::from_utf16_lossy(&buffer[..length as usize])))
        }

        pub fn bounds(&self) -> Result<PlatformBounds, PlatformError> {
            let mut value = MaybeUninit::uninit();

            let rect = unsafe {
                WindowsAndMessaging::GetWindowRect(self.0, value.as_mut_ptr())?;
                value.assume_init()
            };

            Ok(PlatformBounds(rect))
        }

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

    pub struct PlatformBounds(pub(crate) RECT);

    impl PlatformBounds {
        pub fn sys(&self) -> &RECT {
            &self.0
        }

        pub fn x(&self) -> i32 {
            self.0.left
        }

        pub fn y(&self) -> i32 {
            self.0.top
        }

        pub fn width(&self) -> i32 {
            self.0.right - self.0.left
        }

        pub fn height(&self) -> i32 {
            self.0.bottom - self.0.top
        }

        pub const fn left(&self) -> i32 {
            self.0.left
        }

        pub const fn top(&self) -> i32 {
            self.0.top
        }

        pub const fn right(&self) -> i32 {
            self.0.right
        }

        pub const fn bottom(&self) -> i32 {
            self.0.bottom
        }
    }
}

mod error {
    pub use windows::core::Error as PlatformError;
}
