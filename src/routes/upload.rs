use actix_multipart::Multipart;
use actix_web::{HttpResponse, Error, web};
use futures::{StreamExt, TryStreamExt};
use std::path::Path;
use std::ffi::OsStr;
use uuid::Uuid;
use std::io::Write;
use crate::util;
use crate::s3::upload::upload_to_s3;
use crate::structs::HeliumConfig;

pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", util::rewrite_filename(filename));
        let filepath_clone = filepath.clone();


        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();





        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }

        let config = HeliumConfig {
            helium_key: "12345".to_string(),
            helium_s3_host: "http://fujitsu-server:9000".to_string(),
            helium_s3_acc_key: "minioadmin".to_string(),
            helium_s3_sec_key: "minioadmin".to_string()
        };
        upload_to_s3(Path::new(&filepath_clone), filename.parse()?, config).await;
    }

    Ok(HttpResponse::Ok().into())
}