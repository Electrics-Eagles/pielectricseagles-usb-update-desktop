mod readfiles;
mod compress_7z; 

use std::{path::Path, borrow::Borrow};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use substring::Substring;
use crate::readfiles::ReadFile;

/// A struct that can receive data from command line interface
///
/// # Fields
/// 
/// * `files_to_install` - The file path to install
/// * `install_path` - The destination path to install in RPI
/// * `post_update_script_content_file` - The script path after installion 
/// * `pre_update_script_content_file` - The script path before installion
/// * `drive` - path of disk drive to generate image
/// * `password` - set password 7z file in usb stick image
#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(long = "files-to-install", default_value = "")]
    files_to_install: Vec<String>,

    #[arg(long = "install-path", default_value = "")]
    install_path: Vec<String>,

    #[arg(long = "post-update-script-content-file")]
    post_update_script_content_file: String,

    #[arg(long = "pre-update-script-content-file")]
    pre_update_script_content_file: String,

    #[arg(long = "drive", default_value = "/mnt/update")]
    drive: String,

    #[arg(long = "password", default_value = "12994393")]
    password: String,
}

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
pub struct UpdateStart {
    #[serde(rename = "file_name")]
    pub file_name: String,
    pub checksum: String,
    #[serde(rename = "vendor_id")]
    pub vendor_id: String,
    #[serde(rename = "product_id")]
    pub product_id: String,
}

/// Main code
fn main() {
    /* Receive parameters and commands from CLI */
    let args = Args::parse();

    /* Create directory */
    if Path::new("encrypted_update").is_dir() == false
    {
        std::fs::create_dir("encrypted_update").unwrap();
        std::fs::create_dir("encrypted_update/update").unwrap();
    }

    /* Create json objects */
    let mut list_file_to_update: Vec<Update> = Vec::new();

    /* Iterate list files for updating (reading paths and location of file) */
    for current_position in 0..args.files_to_install.len() {
        /* Read file for getting checksum SHA-256 */
        let file:ReadFile = ReadFile::read_file(args.files_to_install[current_position].clone());
        /* Version str */
        let version_str = String::from("1.0.0");
        /* Generate data type for adding in json file  */
        let update_data :Update = Update {
            location_file: file.filename.clone(),
            checksum: file.checksum.clone(),
            version: version_str,
            install_path: String::from(&args.borrow().install_path[current_position])
        };
        /* Add to json script about update files */
        list_file_to_update.push(update_data);

        /* Copying file to update directory */
        let mut destination_path = String::from("encrypted_update/update/");
        destination_path.push_str(file.filename.clone().as_str());
        std::fs::copy(file.path.clone().as_str(), destination_path).unwrap();
    }

    /* Make struct commom json - file (e.g. "update" ) */
    let root_of_update_file = Root {
        update: list_file_to_update
    };

    /* Write json file update.json */
    let json_string = serde_json::to_string_pretty(&root_of_update_file).unwrap();
    let mut json_update_file = File::create("encrypted_update/update.json").unwrap();
    json_update_file.write_all(json_string.as_bytes()).unwrap();

    /* Copy scripts to encrypted_update directory */
    let mut before_update_path_string = String::from("encrypted_update/");
    before_update_path_string.push_str(ReadFile::read_file(args.pre_update_script_content_file.clone()).filename.as_str());

    let mut after_update_path_string = String::from("encrypted_update/");
    after_update_path_string.push_str(ReadFile::read_file(args.post_update_script_content_file.clone()).filename.as_str());

    std::fs::copy(args.post_update_script_content_file.clone(),after_update_path_string.as_str()).unwrap();
    std::fs::copy(args.pre_update_script_content_file.clone(),before_update_path_string.as_str()).unwrap();

    /* Substring password field */
    let verdor_id_str =  args.password.substring(0,4).clone();
    let product_id_str = args.password.substring(4, 8).clone();
    println!("Verdor ID: {}, Product ID: {}", verdor_id_str, product_id_str);

    /* Make a 7z file via command */
    compress_7z::compress_file_with_password("encrypted_update", "encrypted_update.7z", args.password.clone().as_str());

    /* Reads 7z file before copying to USB-stick for recieving checksum SHA - 256 of this file */
    let _compressed_file = ReadFile::read_file(String::from("encrypted_update.7z"));
    /* Write json file start_update.json */
    let _start_to_update_data = UpdateStart
    {
        file_name: _compressed_file.clone().filename,
        checksum: _compressed_file.clone().checksum,
        vendor_id: String::from(verdor_id_str),
        product_id: String::from(product_id_str)
    };
    
    /* Write json file start_update.json */
    let mut json_start_update_file = File::create("start_update.json").unwrap();
    json_start_update_file.write_all(serde_json::to_string_pretty(&_start_to_update_data).unwrap().as_bytes()).unwrap();

    /* Copy to USB - drive */
    let mut destination_path_to_paste_with_compressed_file: String = args.drive.clone();
    destination_path_to_paste_with_compressed_file.push_str("/encrypted_update.7z");
    let mut destination_path_to_past_with_initial_script_file: String = args.drive.clone();
    destination_path_to_past_with_initial_script_file.push_str("/start_update.json");
    std::fs::copy("encrypted_update.7z", destination_path_to_paste_with_compressed_file).unwrap();
    std::fs::copy("start_update.json", destination_path_to_past_with_initial_script_file).unwrap();

    /* Remove temporary file where near own binaries */
    std::fs::remove_dir_all("encrypted_update").unwrap();
    std::fs::remove_file("encrypted_update.7z").unwrap();
    std::fs::remove_file("start_update.json").unwrap();

}
