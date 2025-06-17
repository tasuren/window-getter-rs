mod bounds;
mod error;
pub mod platform_impl;
mod window;
mod window_id;

pub use bounds::Bounds;
pub use error::Error;
pub use window::Window;
pub use crate::window_id::WindowId;

/// Retrieves a window by its unique identifier.
pub fn get_window(id: WindowId) -> Result<Option<Window>, Error> {
    platform_impl::get_window(id.into())
}

/// Retrieves a list of all open windows on the system.
pub fn get_windows() -> Result<Vec<Window>, Error> {
    platform_impl::get_windows()
}
