use windows::{
    Win32::{Foundation::LPARAM, UI::WindowsAndMessaging::IsWindow},
    core::BOOL,
};

use crate::{Error, Window};

pub use error::WindowsError;
pub use window::WindowsWindow;

pub type WindowsBounds = windows::Win32::Foundation::RECT;
pub type WindowsWindowId = windows::Win32::Foundation::HWND;

/// Retrieves a window by its platform-specific identifier ([`HWND`](windows::Win32::Foundation::HWND)).
pub fn get_window(id: WindowsWindowId) -> Option<Window> {
    let hwnd = id;

    if hwnd.is_invalid() || !unsafe { IsWindow(Some(hwnd)) }.as_bool() {
        None
    } else {
        Some(Window(WindowsWindow::new(hwnd)))
    }
}

unsafe extern "system" fn enum_windows_callback(
    hwnd: windows::Win32::Foundation::HWND,
    lparam: LPARAM,
) -> BOOL {
    let windows = unsafe { &mut *(lparam.0 as *mut Vec<Window>) };
    windows.push(Window(WindowsWindow::new(hwnd)));

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
    use windows::Win32::{
        Foundation::{self, HWND, RECT},
        Graphics::Dwm::{DWMWA_EXTENDED_FRAME_BOUNDS, DwmGetWindowAttribute},
        System::Threading,
        UI::WindowsAndMessaging::{self, GetWindowRect},
    };

    use crate::Bounds;

    use super::WindowsError;

    /// Represents a window in the Windows platform.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WindowsWindow(pub(crate) HWND);

    unsafe impl Send for WindowsWindow {}
    unsafe impl Sync for WindowsWindow {}

    impl WindowsWindow {
        /// Creates a new `PlatformWindow` from a raw [`HWND`].
        ///
        /// # Warning
        /// You must ensure that the `hwnd` is a valid window handle. If you pass an invalid handle,
        /// it may lead to errors on methods.
        /// You can use [`get_window`][super::get_window] to safely retrieve a `PlatformWindow`.
        pub fn new(hwnd: HWND) -> Self {
            Self(hwnd)
        }

        /// Returns the raw handle to the window.
        pub fn hwnd(&self) -> HWND {
            self.0
        }

        /// Returns the title of the window.
        pub fn title(&self) -> Result<Option<String>, WindowsError> {
            let mut buffer = [0u16; 256];
            let length = unsafe { WindowsAndMessaging::GetWindowTextW(self.0, &mut buffer) };

            if length == 0 {
                let raw = windows::core::Error::from_thread();

                return match raw.code() {
                    // If the length is 0 and error is success,
                    // it means the window has no title.
                    Foundation::S_OK => Ok(None),
                    _ => Err(raw),
                };
            }

            Ok(Some(String::from_utf16_lossy(&buffer[..length as usize])))
        }

        /// Returns the raw rectangle of the window by [`GetWindowRect`].
        ///
        /// It includes the invisible resize borders.
        /// So it may not be the same as the window rectangle that is actually seen.
        pub fn rect(&self) -> Result<RECT, WindowsError> {
            Ok(unsafe {
                let mut rect = std::mem::zeroed();
                GetWindowRect(self.0, &mut rect)?;
                rect
            })
        }

        /// This will return [`rect`](Self::rect) value wrapped in [`WindowsBounds`](super::WindowsBounds).
        pub fn bounds(&self) -> Result<Bounds, WindowsError> {
            Ok(self.rect()?.into())
        }

        /// Returns the extended frame bounds of the window
        /// by [`DwmGetWindowAttribute`] with [`DWMWA_EXTENDED_FRAME_BOUNDS`].
        pub fn extended_frame_bounds(&self) -> Result<RECT, WindowsError> {
            Ok(unsafe {
                let mut rect: RECT = std::mem::zeroed();
                DwmGetWindowAttribute(
                    self.0,
                    DWMWA_EXTENDED_FRAME_BOUNDS,
                    &mut rect as *mut RECT as _,
                    std::mem::size_of::<RECT>() as _,
                )?;
                rect
            })
        }

        /// Returns the bounds of the window.
        /// This will return [`extended_frame_bounds`](Self::extended_frame_bounds)
        /// value wrapped in [`WindowsBounds`](super::WindowsBounds).
        pub fn visible_bounds(&self) -> Result<Bounds, WindowsError> {
            Ok(self.extended_frame_bounds()?.into())
        }

        /// Returns the process ID of the owner of this window.
        pub fn owner_pid(&self) -> Result<u32, WindowsError> {
            let mut pid = 0;
            let thread =
                unsafe { WindowsAndMessaging::GetWindowThreadProcessId(self.0, Some(&mut pid)) };

            if thread == 0 {
                Err(windows::core::Error::from_thread())
            } else {
                Ok(pid)
            }
        }

        /// Returns the handle to the process that owns this window.
        pub fn owner_process_handle(
            &self,
        ) -> Result<windows::Win32::Foundation::HANDLE, WindowsError> {
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
        pub fn owner_name(&self) -> Result<String, WindowsError> {
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
                return Err(windows::core::Error::from_thread());
            }

            Ok(String::from_utf16_lossy(&buffer[..length as usize]))
        }

        /// Checks if the window is foreground.
        pub fn is_foreground(&self) -> bool {
            self.0 == unsafe { WindowsAndMessaging::GetForegroundWindow() }
        }
    }
}

mod error {
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
