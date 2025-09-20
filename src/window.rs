use crate::{Bounds, Error, WindowId, platform_impl::PlatformWindow};

/// A wrapper around a platform-specific window.
/// This struct provides a cross-platform interface to interact with window properties.
#[derive(Clone, Debug)]
pub struct Window(pub(crate) PlatformWindow);

impl Window {
    /// Creates a new [`Window`] instance from a platform-specific window.
    ///
    /// # Notes
    /// You can get a `Window` instance by using the [`get_window`](crate::get_window) function
    /// or [`get_windows`](crate::get_windows) function so you don't need to create it manually
    /// in most use cases.
    pub fn new(inner: PlatformWindow) -> Self {
        Self(inner)
    }

    /// Retrieves the underlying platform-specific window.
    pub fn platform_window(&self) -> &PlatformWindow {
        &self.0
    }

    /// Consumes the `Window` and returns the underlying platform-specific window.
    pub fn into_platform_window(self) -> PlatformWindow {
        self.0
    }

    /// Returns the unique identifier of the window.
    pub fn id(&self) -> WindowId {
        #[cfg(target_os = "macos")]
        {
            WindowId(self.0.id())
        }
        #[cfg(target_os = "windows")]
        {
            WindowId(self.0.hwnd())
        }
    }

    /// Returns the title of the window.
    ///
    /// # Platform-specific
    /// - **Windows**: If you don't have permission to access the title,
    ///   it will return [`Error::PermissionDenied`](crate::Error::PermissionDenied).
    /// - **macOS**: It will always return [`Ok`]. Apple's documentation does not
    ///   explicitly state this, but it returns `None` when the permission is not granted.
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
        #[cfg(target_os = "macos")]
        {
            Ok(self.0.bounds()?)
        }
        #[cfg(target_os = "windows")]
        {
            Ok(self.0.visible_bounds()?)
        }
    }

    /// Returns the process ID of the window's owner.
    ///
    /// # Platform-specific
    /// - **macOS**: It will always return [`Ok`].
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
