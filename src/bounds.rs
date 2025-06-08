use std::fmt::Debug;

use crate::platform_impl::PlatformBounds;

pub struct Bounds(pub(crate) PlatformBounds);

impl Bounds {
    pub fn inner(&self) -> &PlatformBounds {
        &self.0
    }

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
