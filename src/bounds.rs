use std::fmt::Debug;

use crate::platform_impl::PlatformBounds;

/// A wrapper around the platform-specific bounds of a window.
/// This struct provides a unified interface for accessing the bounds of a window,
pub struct Bounds(pub(crate) PlatformBounds);

impl Bounds {
    /// Creates a new `Bounds` instance from the given platform-specific bounds.
    pub fn new(bounds: PlatformBounds) -> Self {
        Self(bounds)
    }

    /// Returns the underlying platform-specific bounds.
    pub fn inner(&self) -> &PlatformBounds {
        &self.0
    }

    /// Returns the x-coordinate of the top-left corner of the bounds.
    pub fn x(&self) -> f64 {
        #[cfg(target_os = "macos")]
        {
            self.0.x()
        }

        #[cfg(target_os = "windows")]
        {
            self.0.x().into()
        }
    }

    /// Returns the y-coordinate of the top-left corner of the bounds.
    pub fn y(&self) -> f64 {
        #[cfg(target_os = "macos")]
        {
            self.0.y()
        }

        #[cfg(target_os = "windows")]
        {
            self.0.y().into()
        }
    }

    /// Returns the width of the bounds.
    pub fn width(&self) -> f64 {
        #[cfg(target_os = "macos")]
        {
            self.0.width()
        }

        #[cfg(target_os = "windows")]
        {
            self.0.width().into()
        }
    }

    /// Returns the height of the bounds.
    pub fn height(&self) -> f64 {
        #[cfg(target_os = "macos")]
        {
            self.0.height()
        }

        #[cfg(target_os = "windows")]
        {
            self.0.height().into()
        }
    }
}

impl Debug for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bounds")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}
