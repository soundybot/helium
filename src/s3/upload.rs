use std::fs::File;
use s3::region::Region;
use s3::creds::Credentials;
use s3::bucket::Bucket;
use crate::structs::{S3Storage, HeliumConfig, FileUploaded};
use s3::S3Error;
use std::io::Read;
use std::io::Bytes;
use std::convert::TryFrom;
use actix_web::HttpResponse;
use actix_web::web::Bytes as ActixBytes;
use crate::util::read_file;
use std::path::Path;
use futures::Future;
use std::pin::Pin;
use futures::task::{Context, Poll};
use actix_web::body::Body;
use serde_json::Error;


pub async fn upload_to_s3(mut file_bytes: Vec<u8>, filename: String, content_type: &str, config: HeliumConfig) -> Result<HttpResponse, S3Error> {

    // let mut file: File = File::open(&path)?;
    let minio = S3Storage {
        name: "minio".into(),
        region: Region::Custom {
            region: "eu-central-1".into(),
            endpoint: config.helium_s3_host.into(),
        },
        credentials: Credentials::new(Some(&*config.helium_s3_acc_key), Some(&*config.helium_s3_sec_key), None, None, None)?,
        bucket: "rust-s3".to_string(),
        location_supported: false,
    };

    for backend in vec![minio] {
        // Create Bucket in REGION for BUCKET
        let bucket = Bucket::new_with_path_style(&backend.bucket, backend.region, backend.credentials)?;


        // let mut file_bytes = Vec::new();

        //file.read_to_end(&mut file_bytes).unwrap();

        let (_, code) = bucket.put_object_with_content_type(&*filename, &file_bytes, content_type).await?;

        // println!("{}", bucket.presign_get("test_file", 604801)?);
        assert_eq!(200, code);

        // Get the "test_file" contents and make sure that the returned message
        // matches what we sent.
        let (data, code) = bucket.get_object(&*filename).await?;
        if !(file_bytes == data) {};
        assert_eq!(200, code);


        /* if backend.location_supported {
             // Get bucket location
             println!("{:?}", bucket.location().await?);
         }*/

        bucket.put_object_tagging(&*filename, &[("helium-uploaded", "true")]).await?;
        // let (tags, _status) = bucket.get_object_tagging(&*filename).await?;
    }

    let returnable = FileUploaded {
        path: format!("{}", filename),
        message: "File successfully uploaded!".to_string(),
    };


    Ok(HttpResponse::Ok().header("content-type", "application/json").body(Body::from(match serde_json::to_string(&returnable) {
        Ok(result) => result,
        Err(_) => "An error occured while parsing the response!".to_string()
    })))
}

