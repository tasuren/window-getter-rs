mod bounds;
mod error;
pub mod platform_impl;
mod window;

pub use bounds::Bounds;
pub use error::Error;
pub use window::Window;

pub fn get_windows() -> Result<Vec<Window>, Error> {
    platform_impl::get_windows()
}
