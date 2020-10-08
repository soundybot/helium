use crate::enums::PermissionLvl;
use crate::s3::upload::upload_to_s3;
use crate::s3::util::get_default_tags;
use crate::structs::{HeliumConfig, HeliumConfigWrapper};
use crate::util;
use crate::util::build_perm_err;
use actix_web::{HttpRequest, web, HttpResponse};
use actix_multipart::Multipart;
use actix_web::http::Error;
use actix_web::body::Body;
use tokio::stream::StreamExt;

pub async fn save_file(req: HttpRequest, mut payload: Multipart, config: web::Data<HeliumConfigWrapper>) -> Result<HttpResponse, Error> {

    let config = util::get_config_ownership(&config.config);

    let tags = get_default_tags();

    let perm_lvl = util::permissioncheck(&*match util::get_header(req, "helium_key") {
        Ok(value) => value,
        Err(response) => return Ok(response)
    }, &config);
    if !perm_lvl.eq(&PermissionLvl::ADMIN) {
        return Ok(build_perm_err(PermissionLvl::ADMIN, perm_lvl))
    };

    let mut field = match payload.try_next().await {
        Ok(field) => match field {
            Some(field) => { field }
            None => { return Ok(HttpResponse::InternalServerError().body(Body::from("failed at extracting the field!"))); }
        },
        Err(_) => { return Ok(HttpResponse::BadRequest().body(Body::from("Invalid file. Was the file renamed or deleted?"))) }
    };
    let content_type = field.content_disposition().unwrap();
    let filename = content_type.get_filename().unwrap();
    let filename = util::rewrite_filename(filename).clone();


    let mut data_arr = Vec::new();


    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data: actix_web::web::Bytes = chunk.unwrap();
        data_arr.append(&mut data.to_vec());
    }


    let file_ext = match util::get_extension_from_filename(&*filename) {
        Some(extension) => extension,
        None => ""
    };


    let returnable = match upload_to_s3(data_arr, (&filename).to_string(), util::get_content_type(file_ext), config, tags).await {
        Ok(returnable) => returnable,
        Err(err) => HttpResponse::InternalServerError().body(Body::from(format!("upload returned an error!  \n{:?}", err))),
    };


    Ok(returnable)
}