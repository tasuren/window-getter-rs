/// Error types for window-getter-rs.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The error that occurs when the window environment is not found.
    /// This can happen on only macOS.
    #[error("No window environment is running.")]
    NoWindowEnvironment,
    /// platform-specific error that can occur when interacting with the window environment.
    #[error("A platform-specific error occurred: {0}")]
    PlatformSpecificError(#[from] super::platform_impl::PlatformError),
}
