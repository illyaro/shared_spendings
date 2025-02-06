use actix_web::{get, http::header::ContentType, web, HttpResponse};
use serde::Deserialize;
use crate::model::user::dao::{get_all, get_one};

#[derive(Deserialize)]
struct UserId {
    id: Option<String>,
}

#[get("/users")]
pub async fn get(path: web::Query<UserId>) -> HttpResponse {
    let response = match path.into_inner().id {
        Some(id) => {
            if let Some(user) = get_one(id) {
                vec![user]
            } else {
                Vec::new()
            }
        },
        None => get_all(),
    };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(response)
}