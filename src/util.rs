use std::path::Path;
use std::ffi::OsStr;
use uuid::Uuid;
use crate::structs::{HeliumConfig, env_holder, DefaultReturn};
use std::env;
use std::collections::{HashMap, HashSet};
use serde::Deserialize;
use envy::Error;
use std::panic::PanicInfo;
use std::fs::File;
use std::io::Read;
use crate::enums::PermissionLvl;
use actix_web::error::ContentTypeError::ParseError;
use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::HeaderValue;
use actix_web::http::header::ToStrError;
use actix_web::body::Body;


/*
* extract extension from filename, if present
* e.g. testfile.png -> Some("png")
* e.g. Dockerfile -> None
*/
pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

//generate UUID, rename file and regenerate file extension
pub fn rewrite_filename(filename: &str) -> String {
    match get_extension_from_filename(filename) {
        Some(extension) => format!("{}.{}", Uuid::new_v4(), extension),
        None => Uuid::new_v4().to_string()
    }
}


#[derive(Deserialize)]
struct Config {
    bar: Option<String>,
}

pub fn test() {
    match envy::from_env::<Config>() {
        Ok(config) => println!("provided config.bar {:?}", config.bar),
        Err(err) => println!("error parsing config from env: {}", err),
    }
}

pub fn generate_creds_struct() -> HeliumConfig {
    match envy::from_env::<HeliumConfig>() {
        Ok(config) => config,
        Err(err) => alert_env_not_found(err),
    }
}

#[allow(unreachable_code)]
fn alert_env_not_found(err: Error) -> HeliumConfig {
    panic!(format!("Error parsing config from env: {}", err));

    HeliumConfig {
        helium_key: "".to_string(),
        helium_s3_host: "".to_string(),
        helium_s3_acc_key: "".to_string(),
        helium_s3_sec_key: "".to_string(),
    }
}

fn get_env(name: &str) -> String {
    env::var(format!("{}", name))
        .expect(&*format!("Environment variable {} not found", name))
        .parse()
        .expect(&*format!("Couldn't parse {}", name))
}

pub fn read_file(mut file: &File) -> Result<Box<[u8]>, ()> {
    //let mut file = File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    Ok(Box::from(contents))
}

//gets content type per supported file extension
pub fn get_content_type(file_ext: &str) -> &'static str {
    match file_ext {
        //text
        "txt" => "text/plain",
        "js" => "application/javascript",
        "json" => "application/json",
        "css" => "text/css",
        "csv" => "text/csv",
        "html" => "text/html",
        "md" => "text/markdown",
        "rtf" => "text/rtf",
        "xml" => "text/xml",

        //video
        "mp4" => "video/mp4",
        "3gpp" => "video/3gpp",
        "mov" => "video/quicktime",


        //images
        "jpeg" => "image/jpeg",
        "jpg" => "image/jpeg",
        "png" => "image/png",
        "bmp" => "image/bmp",
        "heic" => "image/heic",
        "tiff" => "image/tiff",
        "svg" => "image/svg+xml",

        //fonts
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "woff" => "font/woff",
        "woff2" => "font/woff2",

        //3d models/objects
        "obj" => "model/obj",
        "3mf" => "model/3mf",

        //other stuff
        _ => "application/octet-stream"
    }
}

pub fn permissioncheck(input: &str, helium_config: HeliumConfig) -> PermissionLvl {
    if input == helium_config.helium_key.as_str() {
        return PermissionLvl::ADMIN;
    };
    return PermissionLvl::UNAUTHORIZED;
}

pub fn perm_to_bool(perm_lvl: PermissionLvl) -> bool {
    if perm_lvl == PermissionLvl::ADMIN { return true; }
    return false;
}

pub fn get_header(req: HttpRequest, header_name: &str) -> Result<String, HttpResponse> {
    let req_error = DefaultReturn {
        code: 400,
        message: format!("Bad request. Header {} is missing.", &header_name),
    };
    let conv_error = DefaultReturn {
        code: 400,
        message: format!("Bad request. Header {} couldn't be converted to str.", &header_name),
    };
    let err_response = HttpResponse::BadRequest()
        .header("Content-Type", "application/json")
        .finish();
    let headers = req.headers();
    let header = match headers.get(header_name) {
        Some(header) => match header.to_str() {
            Ok(header) => header,
            Err(_) => return Err(err_response.set_body(Body::from(serde_json::to_string(&conv_error).unwrap())))
        },
        None => return Err(err_response.set_body(Body::from(serde_json::to_string(&req_error).unwrap()))),
    };


    Ok(header.parse().unwrap())
}


pub fn build_perm_err(req_perm: PermissionLvl, real_perm: PermissionLvl) -> HttpResponse {
    let perm_error = DefaultReturn { code: 401, message: format!("Your permission level {} is not greater or equal to the required one: {}", real_perm, req_perm) };
    HttpResponse::Unauthorized().content_type("application/json").body(Body::from(match serde_json::to_string(&perm_error) {
        Ok(perm_error) => perm_error,
        Err(_) => return HttpResponse::InternalServerError().content_type("text/plain").body(Body::from("An error occured while parsing the permission error response!"))
    }))
}