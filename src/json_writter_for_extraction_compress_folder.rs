use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    update: UpdateStart,
}

/// Struct of update body of JSON file (for start_update.json)
///
/// # Fields
/// 
/// * `file_name` - the filename of 7z file
/// * `checksum` - the checksum of 7z file
/// * `verdor_id` - the first of four character password (for example 12345678 is given password. 1234 is verdor_id)
/// * `product_id` - the fitth of four character password  (for example 12345678 is given password. 5678 is product_id)
/// 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "update")]
pub struct UpdateStart {
    #[serde(rename = "file_name")]
    pub file_name: String,
    pub checksum: String,
    #[serde(rename = "vendor_id")]
    pub vendor_id: String,
    #[serde(rename = "product_id")]
    pub product_id: String,
}

pub fn write_general_json_about_zip_file(filepath: String, startupdate_script: UpdateStart)
{
    let main_script = Root{
        update: startupdate_script,
    };
    let mut json_start_update_file = File::create(filepath).unwrap();
    json_start_update_file.write_all(serde_json::to_string_pretty(&main_script).unwrap().as_bytes()).unwrap();
}
