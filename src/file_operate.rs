use std::{path::PathBuf, fs};
use serde::{Serialize, Deserialize};
use std::io::Write;
use crate::common_operate::Address;
use super::common_operate;

const BASE_PATH: &str = "file";
const SUPPORT_FILE_TYPE: [&str; 17] = ["TXT","CSV","LOG","XML","PDF","JSON","JPG","PNG","GIF","BMP","SVG","MP3","OGG","WAV","MP4","WEBM","HTML"];

#[derive(Serialize)]
pub struct CustomFolder {
    folder_name: String,
    folder_files: Vec<CustomFile>
}

impl CustomFolder {
    fn new(folder_name: String, folder_files: Vec<CustomFile>) -> Self {
        CustomFolder { folder_name, folder_files }
    }
}

#[derive(Serialize)]
pub struct CustomFile {
    file_name: String,
    file_path: String,
    link_path: String,
}

impl CustomFile {
    fn new(file_name: String, file_path: String, link_path: String) -> Self {
        CustomFile { file_name, file_path, link_path }
    }
}

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    file_path: String
}

pub fn get_custom_folder(target_file_path: &str) -> CustomFolder {
    let address: Address = common_operate::get_address();
    let link: String = format!("http://{}:{}", address.access_ip, address.access_port);
    let mut files_in_folder:Vec<CustomFile> = Vec::<CustomFile>::new();
    let select_path: String = format!("{0}/{1}", BASE_PATH, target_file_path);
    let file_path_buf: PathBuf = PathBuf::from(select_path);
    let entries = fs::read_dir(file_path_buf.as_path()).expect("Unable to read folder.");
    for entry in entries {
        let entry = entry.expect("Unable to read file.");
        let file_path = entry.path().as_path().to_str().unwrap().to_owned();
        let file_name = entry.file_name().to_os_string().to_str().unwrap().to_owned();
        let mut link_path = String::from("Access not supported.");
        let file_format = file_name.as_str().split_once('.').unwrap().1.to_uppercase();
        let file_format_ref = file_format.as_str();
        if SUPPORT_FILE_TYPE.contains(&file_format_ref) {
            link_path = format!("{0}/{1}/{2}/{3}", link, BASE_PATH, target_file_path,&file_name);
        }
        let custom_file = CustomFile::new(file_name, file_path, link_path);
        files_in_folder.push(custom_file);
    }
    let folder_name = file_path_buf.as_path().file_name().unwrap().to_str().unwrap().to_owned();
    CustomFolder::new(folder_name, files_in_folder)
}

pub fn save_files(file_name: &str, file_path: &str, file_data: Vec<u8>) {
    let target_path: String = format!("{0}/{1}", BASE_PATH,file_path);
    fs::create_dir_all(&target_path).expect(format!("Cannot the path {}",target_path).as_str());
    let target_file_path = PathBuf::from(&target_path).join(file_name);
    let mut target_file: fs::File = fs::File::create(&target_file_path).expect("Unable to create the target file.");
    log::info!("File is saved in {}", target_path);
    target_file
    .write_all(&file_data)
    .expect("Could not write data to destination file.");
}
