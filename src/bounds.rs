use std::fmt::Debug;

use crate::platform_impl::PlatformBounds;

/// A wrapper around the platform-specific bounds of a window.
/// This struct provides a unified interface for accessing the bounds of a window,
#[derive(Debug, Clone, Default)]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[cfg(target_os = "macos")]
impl From<PlatformBounds> for Bounds {
    fn from(value: PlatformBounds) -> Self {
        Bounds {
            x: value.origin.x,
            y: value.origin.y,
            width: value.size.width,
            height: value.size.height,
        }
    }
}
