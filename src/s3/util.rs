#[allow(usused_variables)]
use std::collections::HashMap;

use actix_web::test::config;
use s3::creds::Credentials;
use s3::region::Region;

use crate::structs::{HeliumConfig, S3Storage};
use s3::S3Error;

pub fn check_file_existing(filename: String, config: &HeliumConfig) {}

pub fn build_config_struct(helium_config: &HeliumConfig) -> Result<S3Storage, S3Error> {
    let returnable = S3Storage {
        name: "helium".into(),
        region: Region::Custom {
            region: "h0-atmosphere".into(),
            endpoint: (&*helium_config.helium_s3_host).parse().unwrap(),
        },
        credentials: Credentials::new(Some(&*helium_config.helium_s3_acc_key), Some(&*helium_config.helium_s3_sec_key), None, None, None)?,
        bucket: helium_config.helium_s3_bucket.to_owned(),
        location_supported: false,
    };
    Ok(returnable)
}

//get default tags to tag the uploaded files with
pub fn get_default_tags() -> HashMap<String, String> {
    let mut tags: HashMap<String, String> = HashMap::new();
    tags.insert("helium-uploaded".to_string(), "true".to_string());

    return tags
}