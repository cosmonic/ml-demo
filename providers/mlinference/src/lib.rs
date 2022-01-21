use std::{collections::HashMap};
use wasmcloud_interface_mlinference::{ ResultStatus };
use thiserror::Error as ThisError;

mod settings;
pub use settings::{load_settings, ModelSettings};
mod hashmap_ci;
pub(crate) use hashmap_ci::make_case_insensitive;

pub type BindlePath = String;
pub type ModelName = String;
pub type ModelZoo = HashMap<ModelName, BindlePath>;

/// errors generated by this crate
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("problem reading settings: {0}")]
    Settings(String),

    #[error("provider startup: {0}")]
    Init(String),

    #[error("deserializing settings: {0}")]
    SettingsToml(toml::de::Error),
}

/// generates a valid result
/// TODO__CB__ could be some 'default'?
pub fn get_valid_status() -> ResultStatus {
    ResultStatus {
        has_error: false,
        error: None
    }
}

/// removes all remaining state for given models
/// TODO__CB__
pub fn drop_state(_mz: ModelZoo) -> () {}