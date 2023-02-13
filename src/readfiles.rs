use std::path::Path;
use sha256::try_digest;
use substring::Substring;
/// A struct that can receive information about file
///
/// # Fields
/// 
/// * `filename` - String representing filename (e.g. myfile.txt)
/// * `path` - String representing file path (e.g. /home/linuxpc/Desktop/testmut)
/// * `checksum` - String representing SHA-256 checksum
#[derive(Debug, Clone)]
pub struct ReadFile
{
    pub filename: String,
    pub path: String,
    pub checksum: String
}

/// # Summary
/// 
/// The three important paarmeter any files is path, filename, checksum SHA-256 for detect is fake file
///
/// # Description
/// 
/// You need use the read_file function to order read file
///
/// # Arguments
/// 
/// `path_args` - full path of file, but not path of director. You must use file path like /home/linuxpc/Desktop/testmut/test.7z (NOT /home/linuxpc/Desktop/testmut)!
///
/// # Returns
/// 
/// `ReadFile` - a struct of data about this given file by argrument of function
///
/// # Example
/// ```rust
/// let file: ReadFile = ReadFile::read_file("/home/linuxpc/Desktop/testmut/test.7z");
/// ```
impl ReadFile
{
    pub fn read_file(path_args: String) -> ReadFile
    {
        let input = Path::new(path_args.as_str());
        let val_checksum = try_digest(input).unwrap();

        let is_have_slash_file_path = path_args.rfind("/");

        let mut filename_str= String::new();

        filename_str = match is_have_slash_file_path
        {
            Some(x) =>  String::from(path_args.substring(x + 1, path_args.len())),
            None => path_args.clone()
        };
        
        ReadFile
        {
            filename: filename_str,
            path: path_args,
            checksum: val_checksum.clone()
        } 
    }
}