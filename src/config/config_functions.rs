use std::cell::OnceCell;
use std::error::Error;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use crate::config::config_struct::Config;

lazy_static! {
    pub static ref CONFIG: Mutex<OnceCell<Config>> = Mutex::new(OnceCell::new());
}

pub async fn read_config() -> Result<(), Box<dyn Error>> {

    let config_file = std::fs::read_to_string("./config.toml")?;
    let mut config: Config = toml::from_str(&config_file)?;

    if config.directories.downloads.ends_with('/') {
        config.directories.downloads = config.directories.downloads.trim_end_matches('/').to_string();
    }

    if config.directories.watchfile.ends_with('/') {
        config.directories.watchfile = config.directories.watchfile.trim_end_matches('/').to_string();
    }

    let config_cell = CONFIG.lock().await;
    config_cell.set(config).unwrap();



    Ok(())
}