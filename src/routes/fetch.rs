use actix_web::{HttpRequest, web, HttpResponse};
use crate::structs::{DefaultResponse, HeliumConfigWrapper};
use actix_web::body::Body;
use crate::enums::S3DownloadError;
use crate::util;

pub async fn fetch(req: HttpRequest, config: web::Data<HeliumConfigWrapper>) -> HttpResponse<Body> {

    let config = util::get_config_ownership(&config.config);

    let path = req.match_info().get("path").unwrap().parse().unwrap();

    let download_return = match download_from_s3(path, config).await {
        Ok(res) => match res {
            Ok(res) => res,
            Err(err) => match err {
                S3DownloadError::NotFound => return HttpResponse::NotFound().content_type("application/json").body(match serde_json::to_string(&DefaultResponse { message: "The requested resource could not be found!".to_string() }) {Ok(string) => string, Err(err) => format!("an error occurred while trying to parse the error information: {:?}", err)}),
                _ => return HttpResponse::NotFound().finish()
            }
        },
        Err(err) => return HttpResponse::InternalServerError().content_type("application/json").body(match serde_json::to_string(&DefaultResponse { message: format!("{:?}", err) }) {Ok(string) => string, Err(err) => format!("an error occurred while trying to parse the error information: {:?}", err)})
    };

    let filetype = download_return.content_type;
    let filedata = download_return.data;

    HttpResponse::Ok().content_type(filetype).body(filedata)
}