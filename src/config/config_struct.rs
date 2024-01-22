use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    dirs: Directories,
    dl: Downloading,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Directories {
    watchfile: String,
    downloads: String,
    index: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Downloading {
    parallel_downloads: i32,
}