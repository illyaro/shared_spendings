use actix_web::http::header::ContentType;
use actix_web::{delete, web, HttpResponse};

use crate::model::user::dao::delete;
use crate::model::misc::Info;

#[delete("/users/{id}")]
pub async fn remove(path: web::Path<(String, )>) -> HttpResponse {
    let id = path.into_inner().0;
    let res = delete(&id);
    let msg = format!("Removed {} record with id {}", res, id);

    HttpResponse::Ok()
        .append_header(ContentType::json())
        .json(web::Json(Info::new(msg)))
}