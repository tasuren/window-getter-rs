use crate::{Bounds, Error, platform_impl::PlatformWindow};

pub struct Window(pub(crate) PlatformWindow);

impl Window {
    pub fn inner(&self) -> &PlatformWindow {
        &self.0
    }

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

    pub fn bounds(&self) -> Result<Bounds, Error> {
        #[cfg(target_os = "macos")]
        {
            Ok(self.0.bounds().map(Bounds)?)
        }

        #[cfg(target_os = "windows")]
        {
            Ok(self.0.bounds().map(Bounds)?)
        }
    }

    pub fn owner_pid(&self) -> Result<i32, Error> {
        #[cfg(target_os = "macos")]
        {
            Ok(self.0.owner_pid())
        }

        #[cfg(target_os = "windows")]
        {
            Ok(self.0.owner_pid()?)
        }
    }

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
