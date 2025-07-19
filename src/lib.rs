#![doc = include_str!("../README.md")]

mod bounds;
mod error;
pub mod platform_impl;
mod window;
mod window_id;

pub use crate::window_id::WindowId;
pub use bounds::Bounds;
pub use error::Error;
pub use window::Window;

/// Retrieves a window by its unique identifier.
///
/// # Platform-specific
/// - **windows:** It will always return [`Ok`].
pub fn get_window(id: WindowId) -> Result<Option<Window>, Error> {
    #[cfg(target_os = "macos")]
    {
        platform_impl::get_window(id.into())
    }
    #[cfg(target_os = "windows")]
    {
        Ok(platform_impl::get_window(*id.inner()))
    }
}

/// Retrieves a list of all open windows on the system.
pub fn get_windows() -> Result<Vec<Window>, Error> {
    platform_impl::get_windows()
}
