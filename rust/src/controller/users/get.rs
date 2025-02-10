use crate::{
    controller::authorization_check::verify_token,
    model::user::dao::{get_all, get_one},
};
use actix_web::{get, http::header::ContentType, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserId {
    id: Option<String>,
}

#[get("/users")]
pub async fn get(request: HttpRequest, path: web::Query<UserId>) -> HttpResponse {
    verify_token(request.headers());

    let response = match path.into_inner().id {
        Some(id) => {
            if let Some(user) = get_one(id) {
                vec![user]
            } else {
                Vec::new()
            }
        }
        None => get_all(),
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(response)
}
