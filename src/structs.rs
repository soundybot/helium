use s3::region::Region;
use serde::{Deserialize, Serialize};
use s3::creds::Credentials as s3Credentials;
use actix_web::HttpResponse;


//TODO clarify naming scheme

//Holder for all credencials and connection strings
#[derive(Deserialize, Debug)]
pub struct HeliumConfig {
    //helium
    pub helium_key: String,

    //s3
    pub helium_s3_host: String,
    pub helium_s3_acc_key: String,
    pub helium_s3_sec_key: String,
    pub helium_s3_bucket: String,

}

pub struct HeliumConfigWrapper {
    pub config: HeliumConfig,
    pub api_version: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct DefaultResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct FileUploaded {
    pub path: String,
    pub message: String,
}

pub struct s3Uploaded {
    pub response: HttpResponse,
    pub path: String,
}

#[derive(Serialize)]
pub struct DefaultReturn {
    pub code: i32,
    pub message: String,
}


//used in generating env map
pub struct env_holder {
    pub name: String,
    pub key: String,
}

// s3 connection holder
pub struct S3Storage {
    pub name: String,
    pub region: Region,
    pub credentials: s3Credentials,
    pub bucket: String,
    pub location_supported: bool,
}

#[derive(Deserialize, Debug)]
pub struct DeleteFile {
    pub path: String,
}

pub struct S3DownloadResult {
    pub data: Vec<u8>,
    pub content_type: String,
}

pub struct S3DownloadError {
    pub message: String,
}