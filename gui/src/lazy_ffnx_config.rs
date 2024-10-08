use log::warn;
use mumba_core::game::ffnx_config::FfnxConfig;
use mumba_core::game::ffnx_installation::FfnxInstallation;
use mumba_core::toml::FileError;
use std::path::PathBuf;

pub struct LazyFfnxConfig {
    config: Option<FfnxConfig>,
    config_path: PathBuf,
}

impl LazyFfnxConfig {
    pub fn new(installation: &FfnxInstallation) -> Self {
        Self {
            config: None,
            config_path: installation.config_path(),
        }
    }

    pub fn get(&mut self) -> &mut FfnxConfig {
        if self.config.is_none() {
            self.config = Some(FfnxConfig::from_file(&self.config_path).unwrap_or_default())
        }

        self.config.as_mut().unwrap()
    }

    pub fn get_bool(&mut self, key: &str, default: bool) -> bool {
        match self.get().get_bool(key, default) {
            Ok(v) => v,
            Err(e) => {
                warn!("Get FFNx config entry error: {}", e);
                default
            }
        }
    }

    pub fn get_int(&mut self, key: &str, default: i32) -> i32 {
        match self.get().get_int(key, default as i64) {
            Ok(v) => v as i32,
            Err(e) => {
                warn!("Get FFNx config entry error: {}", e);
                default
            }
        }
    }

    pub fn save(&mut self) -> Result<(), FileError> {
        if let Some(config) = &self.config {
            config.save(&self.config_path)?
        }
        self.clear();
        Ok(())
    }

    pub fn clear(&mut self) {
        self.config = None
    }
}
