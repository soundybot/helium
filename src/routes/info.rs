use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::body::Body;
use actix_web::error::PayloadError::Http2Payload;
use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use futures::{FutureExt, StreamExt, TryStreamExt, Stream};
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
use actix_web::client::{Client, ClientResponse, SendRequestError};
use tokio::macros::support::Pin;
use actix_web::dev::Payload;
use actix_web::error::PayloadError;
use tokio::time::Duration;


#[derive(Serialize)]
struct Info {
    api_version: String,
    version: String,
    minio_code: u16,
}

pub async fn info(req: HttpRequest, config: web::Data<HeliumConfigWrapper>) -> HttpResponse<Body> {

    let returnable = Info {
        api_version: config.api_version.to_owned(),
        version: config.version.to_owned(),
        minio_code: is_minio_up(config.as_ref()).await,
    };

    HttpResponse::Ok().content_type("application/json").body(match serde_json::to_string(&returnable) {Ok(string) => string, Err(err) => format!("an error occurred while trying to parse the application information: {:?}", err)})
}

async fn is_minio_up(config: &HeliumConfigWrapper) -> u16 {

    let mut client = Client::default();

    // Create request builder and send request to minio
    let response = client.get(format!("{}/minio/health/live", config.config.helium_s3_host))
        .header("User-Agent", "helium/1.0")
        .timeout(Duration::from_secs(5))
        .send()     // Send request
        .await;     // Wait for response

    let code = match response {
        Ok(response) => response.status(),
        Err(err) => return 999,
    };

    println!("Response: {:?}", code.as_u16());
    return code.as_u16();
}