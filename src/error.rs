/// Error types for window-getter-rs.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The error that occurs when the window environment is not found.
    /// This can happen on only macOS.
    #[error("No window environment is running.")]
    NoWindowEnvironment,
    /// The error that occurs when you don't have permission to access the window property.
    /// This can happen on Windows.  
    /// It represents [`E_ACCESSDENIED`][hresult] of [`HRESULT`](windows::core::HRESULT).
    ///
    /// [hresult]: <https://learn.microsoft.com/en-us/windows/win32/seccrypto/common-hresult-values>
    #[error("You don't have permission to access the window property: {0}")]
    PermissionDenied(super::platform_impl::PlatformError),
    /// platform-specific error that can occur when interacting with the window environment.
    #[error("A platform-specific error occurred: {0}")]
    PlatformSpecificError(super::platform_impl::PlatformError),
}
