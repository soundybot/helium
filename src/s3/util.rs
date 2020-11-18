#[allow(usused_variables)]
use std::collections::HashMap;

use actix_web::test::config;
use rusoto_core::{Region, RusotoError};

use crate::structs::{HeliumConfig, S3Storage};
use rusoto_s3::{S3Error, S3, CreateBucketRequest, CreateBucketConfiguration, CreateBucketOutput, CreateBucketError};
use rusoto_s3::{Bucket, S3Client};
use futures::Future;

pub fn check_file_existing(filename: String, config: &HeliumConfig) {}
/*
pub fn build_config_struct(helium_config: &HeliumConfig) -> Result<S3Storage, S3Error> {
    let returnable = S3Storage {
        name: "helium".into(),
        region: S3Region::Custom {
            region: "h0-atmosphere".into(),
            endpoint: (&*helium_config.helium_s3_host).parse().unwrap(),
        },
        credentials: Credentials::new(Some(&*helium_config.helium_s3_acc_key), Some(&*helium_config.helium_s3_sec_key), None, None, None)?,
        bucket: helium_config.helium_s3_bucket.to_owned(),
        location_supported: false,
    };
    Ok(returnable)
}*/

pub fn build_s3client(config: &HeliumConfig) -> S3Client {
    S3Client::new(get_region(config))
}
pub fn get_region(config: &HeliumConfig) -> Region {
    Region::Custom {
        name: "h0-atmosphere".to_string(),
        endpoint: config.helium_s3_host.to_owned(),
    }
}


//get default tags to tag the uploaded files with
pub fn get_default_tags() -> HashMap<String, String> {
    let mut tags: HashMap<String, String> = HashMap::new();
    tags.insert("helium-uploaded".to_string(), "true".to_string());

    return tags;
}

pub fn create_bucket(config: &HeliumConfig, region: Region) {
    let client = build_s3client(config);
    let request = CreateBucketRequest {
        acl: Option::from("public-read"),
        bucket: config.helium_s3_bucket.to_owned(),
        ..Default::default()
    };
    let response = match client.create_bucket(request).await {
        Ok(res) => println!("Successfully created bucket in region {}", *res.location),
        Err(err) => println!("Bucket already exists!")
    };
}