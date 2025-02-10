use actix_web::http::header::ContentType;
use actix_web::{delete, web, HttpRequest, HttpResponse};

use crate::controller::authorization_check::verify_token;
use crate::model::misc::Info;
use crate::model::user::dao::delete;

#[delete("/users/{id}")]
pub async fn remove(request: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    verify_token(request.headers());

    let id = path.into_inner().0;
    let res = delete(&id);
    let msg = format!("Removed {} record with id {}", res, id);

    HttpResponse::Ok()
        .append_header(ContentType::json())
        .json(web::Json(Info::new(msg)))
}
