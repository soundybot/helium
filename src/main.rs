use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};
use std::path::Path;
use std::ffi::OsStr;
use uuid::Uuid;
use ::s3::creds::{Credentials, AwsCredsError};
use ::s3::bucket::Bucket;
use ::s3::region::Region;
use crate::util::generate_creds_struct;

mod routes;
mod s3;
mod structs;
mod util;
mod enums;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    std::fs::create_dir_all("./tmp").unwrap();

    let config = util::generate_creds_struct();
    println!("{:?}", &config);

    let ip = "0.0.0.0:3000";

    let config = web::Data::new(util::generate_creds_struct());

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .wrap(middleware::Logger::default())
            .service(
            web::resource("/")
                .route(web::post().to(routes::upload::save_file)),
        )
    })
        .bind(ip)?
        .run()
        .await
}