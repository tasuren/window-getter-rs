use crate::{Bounds, Error, platform_impl::PlatformWindow};

/// A wrapper around a platform-specific window.
/// This struct provides a cross-platform interface to interact with window properties.
pub struct Window(pub(crate) PlatformWindow);

impl Window {
    /// Retrieves the underlying platform-specific window.
    pub fn inner(&self) -> &PlatformWindow {
        &self.0
    }

    /// Returns the title of the window.
    ///
    /// # Platform-specific
    /// - **Windows**: If you don't have permission to access the title,
    ///   it will return [`Error::PermissionDenied`](crate::Error::PermissionDenied).
    /// - **macOS**: It will always return [`Ok`].
    pub fn title(&self) -> Result<Option<String>, Error> {
        #[cfg(target_os = "macos")]
        {
            Ok(self.0.title())
        }

        #[cfg(target_os = "windows")]
        {
            Ok(self.0.title()?)
        }
    }

    /// Returns the bounds of the window.
    pub fn bounds(&self) -> Result<Bounds, Error> {
        Ok(self.0.bounds().map(Bounds)?)
    }

    /// Returns the process ID of the window's owner.
    ///
    /// # Platform-specific
    /// **macOS**: It will always return [`Ok`].
    pub fn owner_pid(&self) -> Result<i32, Error> {
        #[cfg(target_os = "macos")]
        {
            Ok(self.0.owner_pid())
        }

        #[cfg(target_os = "windows")]
        {
            Ok(self.0.owner_pid()? as _)
        }
    }

    /// Returns the name of the process that owns the window.
    ///
    /// # Platform-specific
    /// - **Windows**: If you don't have permission to access the owner name,
    ///   it will return [`Error::PermissionDenied`](crate::Error::PermissionDenied).
    ///   Also, it will return the name of the executable file when owner name is available.
    /// - **macOS**: It will always return [`Ok`].
    pub fn owner_name(&self) -> Result<Option<String>, Error> {
        #[cfg(target_os = "macos")]
        {
            Ok(self.0.owner_name())
        }

        #[cfg(target_os = "windows")]
        {
            Ok(self.0.owner_name().map(Some)?)
        }
    }
}
