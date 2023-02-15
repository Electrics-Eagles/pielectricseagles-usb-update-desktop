use serde::{Deserialize, Serialize};
use std::{path::Path, borrow::Borrow};

/// A struct that retrieve main body of JSON file
///
/// # Fields
/// 
/// * `Vec<Update>` - The list of update sub-scripts
/// 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub update: Vec<Update>,
}
/// Strcut of update body of JSON file (for update.json)
///
/// # Fields
/// 
/// * `location_file` - the filename to install
/// * `checksum` - the checksum of file
/// * `version` - version of file
/// * `install_path` - a directory path to install file in RPI Zero 
/// 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    #[serde(rename = "location_file")]
    pub location_file: String,
    pub checksum: String,
    pub version: String,
    #[serde(rename = "install_path")]
    pub install_path: String,
}
