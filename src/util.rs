use std::path::Path;
use std::ffi::OsStr;
use uuid::Uuid;
use crate::structs::{HeliumConfig, env_holder};
use std::env;
use std::collections::{HashMap, HashSet};
use serde::Deserialize;
use envy::Error;
use std::panic::PanicInfo;
use std::fs::File;
use std::io::Read;


/*
* extract extension from filename, if present
* e.g. testfile.png -> Some("png")
* e.g. Dockerfile -> None
*/
fn get_extension_from_filename(filename: &str) -> Option<&str> {
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