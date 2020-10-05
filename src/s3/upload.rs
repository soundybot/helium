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
use crate::util::read_file;
use std::path::Path;

const MESSAGE: &str = "this is a message";

pub async fn upload_to_s3(mut path: &Path, filename: String, config: HeliumConfig) -> Result<HttpResponse, S3Error> {
    let mut file: File = File::open(&path)?;
    let minio = S3Storage {
        name: "minio".into(),
        region: Region::Custom {
            region: "eu-central-1".into(),
            endpoint: "http://fujitsu-server:9000".into(),
        },
        credentials: Credentials::new(Some("minioadmin"), Some("minioadmin"), None, None, None)?,
        bucket: "rust-s3".to_string(),
        location_supported: false,
    };

    for backend in vec![minio] {
        println!("Running {}", backend.name);
        // Create Bucket in REGION for BUCKET
        let bucket = Bucket::new_with_path_style(&backend.bucket, backend.region, backend.credentials)?;


        // List out contents of directory
      /*  let results = bucket.list("".to_string(), None).await?;
        for (list, code) in results {
            assert_eq!(200, code);
            println!("{:?}", list.contents.len());
        }*/

        let mut file_bytes = Vec::new();

        file.read_to_end(&mut file_bytes).unwrap();

        println!("between");
        let (_, code) = bucket.put_object(&*filename, &file_bytes).await?;

        // println!("{}", bucket.presign_get("test_file", 604801)?);
        assert_eq!(200, code);

        // Get the "test_file" contents and make sure that the returned message
        // matches what we sent.
        let (data, code) = bucket.get_object(&*filename).await?;
        if !(file_bytes == data){};
        assert_eq!(200, code);


        if backend.location_supported {
            // Get bucket location
            println!("{:?}", bucket.location().await?);
        }

       // bucket.put_object_tagging_blocking(&*filename, &[("helium-uploaded")])?;
        let (tags, _status) = bucket.get_object_tagging(&*filename).await?;

    }

    let returnable = FileUploaded {
        path: format!("{}", filename),
        message: "File successfully uploaded!".to_string()
    };



    Ok(HttpResponse::Ok().header("content-type", "application/json").finish())
}

