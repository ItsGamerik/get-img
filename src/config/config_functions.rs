use std::cell::OnceCell;
use std::error::Error;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::config::config_struct::Config;

lazy_static! {
    pub static ref CONFIG: Mutex<OnceCell<Config>> = Mutex::new(OnceCell::new());
}

pub fn read_config() -> Result<(), Box<dyn Error>> {

    let config_file = std::fs::read_to_string("./config.toml")?;
    let config: Config = toml::from_str(&config_file)?;

    let config_cell = CONFIG.lock()?;
    config_cell.set(config).unwrap();

    Ok(())
}