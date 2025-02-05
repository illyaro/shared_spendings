use actix_web::http::header::ContentType;
use actix_web::{delete, web, HttpResponse};

use crate::model::record::dao::delete_one;
use crate::model::misc::Info;

#[delete("/records/{id}")]
pub async fn remove(path: web::Path<(i64, )>) -> HttpResponse {
    let id = path.into_inner().0;
    let res = delete_one(id);
    let msg = format!("Removed {} record with id {}", res, id);

    HttpResponse::Ok()
        .append_header(ContentType::json())
        .json(web::Json(Info::new(msg)))
}