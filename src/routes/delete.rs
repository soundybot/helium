#[allow(unused_variables)]
#[allow(duplicate_code)]

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
use uuid::Uuid;

use crate::enums::PermissionLvl;
use crate::structs::{DefaultReturn, DeleteFile, FileUploaded, HeliumConfig, HeliumConfigWrapper};
use crate::util;
use crate::util::{build_perm_err, compile_public_url};
use rusoto_s3::Bucket;
/*
pub async fn delete_file(req: HttpRequest, config: web::Data<HeliumConfigWrapper>, path: web::Json<DeleteFile>) -> Result<HttpResponse, Error> {
    let config = util::get_config_ownership(&config.config);

    let perm_lvl = util::permissioncheck(&*match util::get_header(req, "helium_key") {
        Ok(value) => value,
        Err(response) => return Ok(response)
    }, &config);
    if !perm_lvl.eq(&PermissionLvl::ADMIN) {
        return Ok(build_perm_err(PermissionLvl::ADMIN, perm_lvl));
    };

    let helium_s3 = match  build_config_struct(&config){
        Ok(helium_s3) => helium_s3,
        Err(err) => return Ok(HttpResponse::InternalServerError().body(Body::from(format!("upload returned an error!  \n{:?}", err)))),
    };


    for backend in vec![helium_s3] {
        // Create Bucket in REGION for BUCKET
        let bucket = match Bucket::new_with_path_style(&backend.bucket, backend.region, backend.credentials){
            Ok(bucket) => bucket,
            Err(err) => return Ok(HttpResponse::InternalServerError().content_type("text/plain").body(Body::from(format!("upload returned an error!  \n{:?}", err)))),
        };


        let (del , code) = match bucket.get_object(&path.path).await {
            Ok((data, code)) => ((data, code)),
            Err(s3_err) => return Ok(HttpResponse::InternalServerError().content_type("text/plain").body(Body::from(format!("delete returned an error!  \n{:?}", s3_err)))),
        };

        if code == 404 {
        let returnable = DefaultReturn {
            code: 404,
            message: format!("The requested resource {} cannot be found and therefore wasn't deleted!", &path.path)
        };
        return Ok(HttpResponse::NotFound().content_type("application/json").body(Body::from(serde_json::to_string(&returnable)?)))
        }


        let (del , code) = match bucket.delete_object(&path.path).await {
            Ok((data, code)) => ((data, code)),
            Err(s3_err) => return Ok(HttpResponse::InternalServerError().content_type("text/plain").body(Body::from(format!("delete returned an error!  \n{:?}", s3_err)))),
        };
    }

    Ok(HttpResponse::NoContent().body(format!("File {} has successfully been deleted!", &path.path)))
}*/