use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct AppConfig {
    input: PathBuf,
    assets_output: PathBuf,
    metadata_output: PathBuf,
    order: Vec<PathBuf>,
}

impl AppConfig {
    fn new() -> Self {
        let figment = Figment::new().merge(Toml::file("App.toml"));

        let app_config: AppConfig = figment.extract().expect("Can't extract");

        app_config
    }
}
