use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::body::Body;
use actix_web::error::PayloadError::Http2Payload;
use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use futures::{FutureExt, StreamExt, TryStreamExt};
use s3::bucket::Bucket;
use s3::S3Error;
use uuid::Uuid;
use serde::Serialize;

use crate::enums::PermissionLvl;
use crate::s3::upload::upload_to_s3;
use crate::s3::util::build_config_struct;
use crate::structs::{DefaultReturn, DeleteFile, FileUploaded, HeliumConfig, HeliumConfigWrapper};
use crate::util;
use crate::util::{build_perm_err, compile_public_url};


#[derive(Serialize)]
struct Info {
    api_version: String,
    version: String,
}

pub async fn info(req: HttpRequest, config: web::Data<HeliumConfigWrapper>) -> HttpResponse<Body> {

    let returnable = Info {
        api_version: config.api_version.to_owned(),
        version: config.version.to_owned()
    };


    HttpResponse::Ok().content_type("application/json").body(match serde_json::to_string(&returnable) {Ok(string) => string, Err(err) => format!("an error occurred while trying to parse the application information: {:?}", err)})
}