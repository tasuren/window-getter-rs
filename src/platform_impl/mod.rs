//! Platform-specific implementations for window.

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub use macos::{
    PlatformBounds, PlatformError, PlatformWindow, PlatformWindowId, get_window, get_windows,
};
#[cfg(target_os = "windows")]
pub use windows::{
    PlatformBounds, PlatformError, PlatformWindow, PlatformWindowId, get_window, get_windows,
};
