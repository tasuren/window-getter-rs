use crate::platform_impl::PlatformWindowId;

/// A unique identifier for a window.
/// It is used to track windows across different platforms.
///
/// # Platform-specific
/// - **Windows**: The ID is a value of [`HWND`](windows::Win32::Foundation::HWND).
/// - **macOS**: The ID is a unique within the current user session. It is called a window number.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct WindowId(pub(crate) PlatformWindowId);

unsafe impl Send for WindowId {}
unsafe impl Sync for WindowId {}

impl WindowId {
    pub const fn new(id: PlatformWindowId) -> Self {
        Self(id)
    }

    /// Returns the underlying platform-specific window identifier.
    pub fn inner(&self) -> &PlatformWindowId {
        &self.0
    }

    /// Converts the [`WindowId`] to a [`u32`].
    ///
    /// # Platform-specific
    /// - **macOS**: Returns the window number. It is same as [`WindowId::inner`].
    /// - **Windows**: Returns the window handle as a `u32`.
    pub fn as_u32(&self) -> u32 {
        #[cfg(target_os = "macos")]
        {
            self.0
        }
        #[cfg(target_os = "windows")]
        {
            self.0.0 as _
        }
    }
}

impl From<u32> for WindowId {
    fn from(id: u32) -> Self {
        #[cfg(target_os = "macos")]
        {
            Self(id)
        }
        #[cfg(target_os = "windows")]
        {
            Self(PlatformWindowId(id as _))
        }
    }
}

impl std::hash::Hash for WindowId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_u32().hash(state);
    }
}
