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
    println!("starting upload...");

    let helium_s3 = build_config_struct(config.borrow())?;

    for backend in vec![helium_s3] {
        println!("before bucket creation");
        // Create Bucket in REGION for BUCKET
        let bucket = Bucket::new_with_path_style(&backend.bucket, backend.region, backend.credentials)?;

        println!("after bucket creation");
        println!("content-type: {content} \n filename: {filename} \n config: {config:?}", content=content_type, filename=filename, config=config);

        let (_, code) = match bucket.put_object_with_content_type(&*filename, &file_bytes, content_type).await {
            Ok(a) => a,
            Err(err) => { println!("error while uploading: \n{:?}", err);
            return Ok(HttpResponse::InternalServerError().body(Body::from(format!("an error occurred while uploading the file! \n {:?}", err))))}
        };
        println!("after upload");

        println!("return code is {}", code);
        assert_eq!(200, code);

        println!("start tagging object");

        match bucket.put_object_tagging(&*filename, &[("helium-uploaded", "true")]).await {
            Ok(a) => a,
            Err(err) => return Ok(HttpResponse::InternalServerError().body(Body::from(format!("an error occurred while uploading the file! \n {:?}", err))))
        };
        println!("after tagging object");
    }

    println!("creating response msg");
    let returnable = FileUploaded {
        path: format!("{}", compile_public_url(config.helium_s3_host.to_owned(), config.helium_s3_bucket.to_owned(), filename.to_owned())),
        message: "File successfully uploaded!".to_string(),
    };

    println!("returning response");
        Ok(HttpResponse::Ok().header("content-type", "application/json").body(Body::from(match serde_json::to_string(&returnable){
        Ok(result) => result,
        Err(_) => "An error occurred while parsing the response!".to_string()
    })))
}