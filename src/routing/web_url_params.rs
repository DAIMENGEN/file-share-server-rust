use serde::{Serialize, Deserialize};
use actix_multipart::form::{MultipartForm, text::Text, tempfile::TempFile};


#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    pub file_path: String
}


#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "filename")]
    pub files: Vec<TempFile>,
    #[multipart(rename = "filepath")]
    pub file_path: Text<String>
}