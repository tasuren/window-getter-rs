#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub use macos::{PlatformBounds, PlatformError, PlatformWindow, get_windows};
#[cfg(target_os = "windows")]
pub use windows::{PlatformBounds, PlatformError, PlatformWindow, get_windows};
