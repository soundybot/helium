use crate::s3::util::build_config_struct;
use s3::bucket::Bucket;
use actix_web::HttpResponse;
use crate::structs::{HeliumConfig, S3DownloadResult};
use std::collections::HashMap;
use s3::S3Error;
use std::borrow::Borrow;
use crate::enums::S3DownloadError as S3DownloadErrorEnum;

pub async fn download_from_s3(filepath: String, config: HeliumConfig) -> Result<Result<S3DownloadResult, S3DownloadErrorEnum>, S3Error> {

    let helium_s3 = build_config_struct(config.borrow())?;
        // Create Bucket in REGION for BUCKET
        let bucket = Bucket::new_with_path_style(&helium_s3.bucket, helium_s3.region, helium_s3.credentials)?;

        let (filedata, code) = match bucket.get_object(filepath).await {
            Ok(a) => a,
            Err(err) => return Ok(Err(S3DownloadErrorEnum::NotFound))
        };


        assert_eq!(200, code);

    let filetype  = tree_magic::from_u8(&filedata);
    println!("{}", filetype);

    let returnable = S3DownloadResult {
        data: filedata,
        content_type: filetype.to_string()
    };
    Ok(Ok(returnable))
}