use actix_web::{web, Result, Responder, post, Error, HttpResponse};
use actix_multipart::form::MultipartForm;
use std::io::prelude::*;
use crate::file_operate::CustomFolder;

use super::web_url_params::{QueryParams, UploadForm};
use super::super::file_operate;

#[post("file_list")]
pub async fn file_list(params: web::Form<QueryParams>) -> Result<impl Responder> {
    let file_path: &String = &params.file_path;
    let custom_folder: CustomFolder = file_operate::get_custom_folder(file_path);
    Ok(web::Json(custom_folder))
}


#[post("file_upload")]
pub async fn file_upload(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    for temp_file in form.files {
        let file_path = &form.file_path.0;
        let ref file_name = temp_file.file_name.unwrap();
        let mut upload_file = temp_file.file;
        let mut file_data = Vec::<u8>::with_capacity(temp_file.size);
        upload_file
        .read_to_end(&mut file_data)
        .expect(format!("Cannot read {}", &upload_file.into_temp_path().to_str().unwrap().to_owned()).as_str());
        file_operate::save_files(file_name, file_path, file_data)
    }

    Ok(HttpResponse::Ok())
}
