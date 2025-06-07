#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No window environment is running.")]
    NoWindowEnvironment,
    #[error("A platform-specific error occurred: {0}")]
    PlatformSpecificError(#[from] super::platform_impl::PlatformError),
}
