//! Platform-specific implementations for window.

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub use macos::{
    MacOSBounds as PlatformBounds, MacOSError as PlatformError, MacOSWindow as PlatformWindow,
    MacOSWindowId as PlatformWindowId, get_window, get_windows,
};
#[cfg(target_os = "windows")]
pub use windows::{
    WindowsBounds as PlatformBounds, WindowsError as PlatformError,
    WindowsWindow as PlatformWindow, WindowsWindowId as PlatformWindowId, get_window, get_windows,
};
