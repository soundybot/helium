
use crate::s3::util::build_config_struct;
use crate::structs::{FileUploaded, HeliumConfig, S3Storage};
use crate::util::{compile_public_url, read_file};
use std::collections::HashMap;
use actix_web::HttpResponse;
use s3::S3Error;
use std::borrow::Borrow;
use s3::bucket::Bucket;
use actix_web::body::Body;

pub async fn upload_to_s3(file_bytes: Vec<u8>, filename: String, content_type: &str, config: HeliumConfig, tags: HashMap<String, String>) -> Result<HttpResponse, S3Error> {

    let helium_s3 = build_config_struct(config.borrow())?;

    for backend in vec![helium_s3] {
        // Create Bucket in REGION for BUCKET
        let bucket = Bucket::new_with_path_style(&backend.bucket, backend.region, backend.credentials)?;


        let (_, code) = bucket.put_object_with_content_type(&*filename, &file_bytes, content_type).await?;

        assert_eq!(200, code);

        let (data, code) = bucket.get_object(&*filename).await?;
        if !(file_bytes == data) {};
        assert_eq!(200, code);

        bucket.put_object_tagging(&*filename, &[("helium-uploaded", "true")]).await?;

    }

    let returnable = FileUploaded {
        path: format!("{}", compile_public_url(config.helium_s3_host.to_owned(), config.helium_s3_bucket.to_owned(), filename.to_owned())),
        message: "File successfully uploaded!".to_string(),
    };


    Ok(HttpResponse::Ok().header("content-type", "application/json").body(Body::from(match serde_json::to_string(&returnable) {
        Ok(result) => result,
        Err(_) => "An error occured while parsing the response!".to_string()
    })))
}