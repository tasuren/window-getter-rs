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

#[cfg(target_os = "macos")]
impl From<Bounds> for PlatformBounds {
    fn from(value: Bounds) -> Self {
        PlatformBounds {
            origin: objc2_core_foundation::CGPoint::new(value.x, value.y),
            size: objc2_core_foundation::CGSize::new(value.width, value.height),
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

#[cfg(target_os = "windows")]
impl From<Bounds> for PlatformBounds {
    fn from(value: Bounds) -> Self {
        PlatformBounds {
            left: value.x as _,
            top: value.y as _,
            right: (value.x + value.width) as _,
            bottom: (value.y + value.height) as _,
        }
    }
}
