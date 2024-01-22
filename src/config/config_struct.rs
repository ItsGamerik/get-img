use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub directories: Directories,
    pub downloading: Downloading,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Directories {
    pub watchfile: String,
    pub downloads: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Downloading {
    pub parallel_downloads: usize,
}