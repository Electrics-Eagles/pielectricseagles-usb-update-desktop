mod readfiles;
mod compress_7z; 
mod json_writter_for_extraction_compress_folder;
mod json_writter_for_updating_file;
use json_writter_for_extraction_compress_folder::*;
use json_writter_for_updating_file::*;
use std::{path::Path, borrow::Borrow};
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
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

    #[arg(long = "pid", default_value = "ffff")]
    pid: String,

    #[arg(long = "vid", default_value = "ffff")]
    vid: String,
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

    /* Receive vid and pid hex value */
    let _vid = u16::from_str_radix(&args.vid, 16);
    let _pid = u16::from_str_radix(&args.pid, 16);

    /* Checkout of assigning value correct */
    let mut _vid_u16 = 0;
    let mut _pid_u16 = 0;
    match _vid {
        Ok(_vid) => {_vid_u16 = _vid; println!("VID : {}", _vid); }, /* The VID will a decimal system digits in console */
        Err(e) => println!("Error: {}", e),  /* If has error about typing VID will be informed */
    };

    match _pid {
        Ok(_pid) => {_pid_u16 = _pid; println!("PID : {}", _pid);}, /* The PID will a decimal system digits in console */
        Err(e) => println!("Error: {}", e), /* If has error about typing PID will be informed */
    };
    /* Make password of 7z file extraction */
    let mut password_str = String::new();
    password_str.push_str(_vid_u16.to_string().as_str());
    password_str.push_str(_pid_u16.to_string().as_str());

    /* Make a 7z file via command */
    compress_7z::compress_file_with_password("encrypted_update", "encrypted_update.7z", &password_str);

    /* Reads 7z file before copying to USB-stick for recieving checksum SHA - 256 of this file */
    let _compressed_file = ReadFile::read_file(String::from("encrypted_update.7z"));
    /* Write json file start_update.json */
    let _start_to_update_data = UpdateStart
    {
        file_name: _compressed_file.clone().filename,
        checksum: _compressed_file.clone().checksum,
        vendor_id: String::from(_vid_u16.to_string()),
        product_id: String::from(_pid_u16.to_string())
    };
    
    /* Write json file start_update.json */
    write_general_json_about_zip_file(String::from("start_update.json"), _start_to_update_data);
    
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
