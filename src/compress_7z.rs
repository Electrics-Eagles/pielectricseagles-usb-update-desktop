use std::process::Command;
/// # Description
/// todo: xcompress ->  7z x -p aaa
/// Compress 7z forder with password protected from our forder with files. When you use this function. You need to install package xcompress before usage. 
/// You could find about xcompress in https://github.com/magiclen/xcompress . 
///
/// # Arguments
///
/// * `forder_path_input` - The path of folder that the function will compress 7z file
/// * `compressed_forder_path` - The path of compressed file that the destination will located when this file is ready
/// * `password` - Set password when extract 7z file
///
/// # Returns
///
/// The function can return zero and minus one. -1 when compressing file error. 0 when finish to compress file with OKAY status. Data type of returning value is i8
///
/// # Example
///
/// ```
/// let compressed_file_status = compress_7z::compress_file_with_password("encrypted_update", "encrypted_update.7z", args.borrow().password.clone().as_str());
/// ```
pub fn compress_file_with_password(forder_path_input: &str, compressed_forder_path: &str, password: &str) -> i8
{
    let mut cmd =Command::new("xcompress"); // Build command for compression 7z file with password
    cmd.arg("a");
    cmd.arg("-p");
    cmd.arg(password);
    cmd.arg(forder_path_input);
    cmd.arg("-o");
    cmd.arg(compressed_forder_path);

    let return_val = match cmd.output() // Check status of making archive 7z zip with nofication in console
    {
        Ok(o) => 0, // If OKAY 
        Err(e) => -1 // If receive error about command
    };

    // Return value
    return_val
}


