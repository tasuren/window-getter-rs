use std::fmt::Debug;

use crate::platform_impl::PlatformBounds;

/// Represents the bounds of a window.
/// It can be converted from platform-specific bounds types.
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

#[cfg(target_os = "windows")]
impl From<PlatformBounds> for Bounds {
    fn from(value: PlatformBounds) -> Self {
        Self {
            x: value.left as _,
            y: value.top as _,
            width: (value.right - value.left) as _,
            height: (value.bottom - value.top) as _,
        }
    }
}
