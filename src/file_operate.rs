use std::{path::PathBuf, fs};
use std::io::prelude::*;
use actix_multipart::{Multipart, form::text::Text};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::{web, Result, Responder, post, Error, HttpResponse};
use serde::{Serialize, Deserialize};
use std::io::Write;
use futures_util::TryStreamExt as _;
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

#[post("/file_list")]
pub async fn file_list(params: web::Form<QueryParams>) -> Result<impl Responder> {
    let address = common_operate::get_address();
    let link = format!("http://{}:{}", address.access, address.port);
    let mut files_in_folder = Vec::<CustomFile>::new();
    let select_path = format!("{0}/{1}", BASE_PATH, &params.file_path);
    let file_path_buf = PathBuf::from(select_path);
    let entries = fs::read_dir(file_path_buf.as_path()).expect("unable to read folder.");
    for entry in entries {
        let entry = entry.expect("unable to read file.");
        let file_path = entry.path().as_path().to_str().unwrap().to_owned();
        let file_name = entry.file_name().to_os_string().to_str().unwrap().to_uppercase();
        let mut link_path = String::from("access not supported.");
        let file_format = file_name.as_str().split_once('.').unwrap().1;
        // log::info!("KEY = {}", &&file_name.to_uppercase().as_str());
        if SUPPORT_FILE_TYPE.contains(&file_format) {
            link_path = format!("{0}/{1}/{2}/{3}", link, BASE_PATH, &params.file_path,&file_name);
        }
        // if file_name.ends_with("html") || file_name.ends_with("pdf") || file_name.ends_with("txt") {
        //     link_path = format!("{0}/{1}/{2}/{3}", link, BASE_PATH, &params.file_path,&file_name);
        // }
        let custom_file = CustomFile::new(file_name, file_path, link_path);
        files_in_folder.push(custom_file);
    }
    let folder_name = file_path_buf.as_path().file_name().unwrap().to_str().unwrap().to_owned();
    let custom_folder = CustomFolder::new(folder_name, files_in_folder);
    Ok(web::Json(custom_folder))
}

#[post("/file_upload_bak")]
async fn file_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let file_name: String = content_disposition.get_filename().unwrap().to_owned();
        let file_path: String = format!("D:/RustProjects/file-share-server/static/{file_name}");
        let mut file: fs::File = web::block(|| fs::File::create(file_path)).await??;
        while let Some(chunk) = field.try_next().await? {
            file = web::block(move || file.write_all(&chunk).map(|_| file)).await??;
        }
    }
    Ok(HttpResponse::Ok().into())
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "filename")]
    files: Vec<TempFile>,
    #[multipart(rename = "filepath")]
    file_path: Text<String>
}

#[post("/file_upload")]
async fn save_files(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    for temp_file in form.files {
        let target_path: String = format!("{0}/{1}", BASE_PATH,form.file_path.0);
        fs::create_dir_all(&target_path).expect(format!("cannot the path {}",target_path).as_str());
        let target_file_path = PathBuf::from(&target_path).join(temp_file.file_name.unwrap());
        let mut target_file: fs::File = fs::File::create(&target_file_path).expect("unable to create the target file.");
        log::info!("File is saved in {}", target_path);
        let mut upload_file = temp_file.file;
        let mut upload_file_data = Vec::<u8>::with_capacity(temp_file.size);
        upload_file
        .read_to_end(&mut upload_file_data)
        .expect(format!("cannot read {}", &upload_file.into_temp_path().to_str().unwrap().to_owned()).as_str());
        target_file
        .write_all(&upload_file_data)
        .expect("Could not write data to destination file.");
    }
    Ok(HttpResponse::Ok())
}


