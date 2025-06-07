#[cfg(target_os = "macos")]
mod macos;

pub use macos::{PlatformBounds, PlatformError, PlatformWindow, get_windows};
