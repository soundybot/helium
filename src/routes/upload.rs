use actix_multipart::{Multipart, Field, MultipartError};
use actix_web::{HttpResponse, Error, web, HttpRequest};
use futures::{StreamExt, TryStreamExt, FutureExt};
use std::path::Path;
use std::ffi::OsStr;
use uuid::Uuid;
use std::io::Write;
use crate::util;
use crate::s3::upload::upload_to_s3;
use crate::structs::HeliumConfig;
use actix_web::web::Bytes;
use s3::S3Error;
use actix_web::body::Body;
use actix_web::error::PayloadError::Http2Payload;
use crate::enums::PermissionLvl;
use crate::util::build_perm_err;

pub async fn save_file(req: HttpRequest, mut payload: Multipart, config: web::Data<HeliumConfig>) -> Result<HttpResponse, Error> {
    /*let config = HeliumConfig {
        helium_key: "12345".to_string(),
        helium_s3_host: "http://fujitsu-server:9000".to_string(),
        helium_s3_acc_key: "minioadmin".to_string(),
        helium_s3_sec_key: "minioadmin".to_string(),
    };*/

    let config = config.get_ref();

    let config = HeliumConfig {
        helium_key: config.helium_key.to_string(),
        helium_s3_host: config.helium_s3_host.to_string(),
        helium_s3_acc_key: config.helium_s3_acc_key.to_string(),
        helium_s3_sec_key: config.helium_s3_sec_key.to_string()
    };

    let perm_lvl = util::permissioncheck(&*match util::get_header(req, "helium_key") {
        Ok(value) => value,
        Err(response) => return Ok(response)
    }, config);
    if !perm_lvl.eq(&PermissionLvl::ADMIN) {
        return Ok(build_perm_err(PermissionLvl::ADMIN, perm_lvl))
    };

    let mut field = match payload.try_next().await {
        Ok(field) => match field {
            Some(field) => { field }
            None => { return Ok(HttpResponse::InternalServerError().body(Body::from("failed at extracting the field!"))); }
        },
        Err(_) => { panic!("field extraction failed!") }
    };
    let content_type = field.content_disposition().unwrap();
    let mut filename = content_type.get_filename().unwrap();
    let filename = util::rewrite_filename(filename).clone();


    let mut data_arr = Vec::new();


    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let mut data: actix_web::web::Bytes = chunk.unwrap();
        data_arr.append(&mut data.to_vec());
        // filesystem operations are blocking, we have to use threadpool
        //  f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }

    let config = HeliumConfig {
        helium_key: "12345".to_string(),
        helium_s3_host: "http://fujitsu-server:9000".to_string(),
        helium_s3_acc_key: "minioadmin".to_string(),
        helium_s3_sec_key: "minioadmin".to_string(),
    };

    let file_ext = match util::get_extension_from_filename(&*filename) {
        Some(extension) => extension,
        None => ""
    };


    let mut returnable = match upload_to_s3(data_arr, (&filename).to_string(), util::get_content_type(file_ext), config).await {
        Ok(returnable) => returnable,
        Err(err) => HttpResponse::InternalServerError().body(Body::from(format!("upload returned an error!  \n{:?}", err))),
    };


    Ok(returnable)
}