#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use ::s3::bucket::Bucket;
use ::s3::creds::{AwsCredsError, Credentials};
use ::s3::region::Region;
use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

use crate::structs::HeliumConfigWrapper;
use crate::util::generate_creds_struct;
use actix_web::error::ParseError::Header;

mod enums;
mod routes;
mod s3;
mod structs;
mod util;

//Version
const API_VERSION: &str = "v1";
const HELIUM_VERSION: &str = "0.2.0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    let config = util::generate_creds_struct();
    println!("{:?}", &config);

    let ip = "0.0.0.0:8080";
    println!("Listening on {}", ip);

    let config = HeliumConfigWrapper {
        config,
        api_version: API_VERSION.to_string(),
        version: HELIUM_VERSION.to_string(),
    };
    let config = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .wrap(middleware::Logger::default())
            //upload file
            .service(
                web::resource("/file")
                    .route(web::post().to(routes::upload::save_file))
                    .route(web::delete().to(routes::delete::delete_file)),
            )
            .service(web::resource("/file/{path}").route(web::get().to(routes::fetch::fetch)))
            .service(web::resource("/api").route(web::get().to(routes::info::info)))
    })
    .bind(ip)?
    .run()
    .await
}
