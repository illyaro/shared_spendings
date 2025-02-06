use actix_web::{http::header::ContentType, put, web, HttpResponse};
use futures::StreamExt;

use crate::model::{misc::Info, user::dao::update};

#[put("/users")]
pub async fn edit(mut payload: web::Payload) -> HttpResponse {
    const MAX_SIZE: usize = 262_144; // 256 KB

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                if chunk.len() + body.len() > MAX_SIZE {
                    return HttpResponse::BadRequest()
                        .json(web::Json(Info::new(String::from("Body too large."))));
                }
                body.extend_from_slice(&chunk)
            }
            Err(err) => {
                return HttpResponse::BadRequest().json(web::Json(Info::new(err.to_string())))
            }
        }
    }

    let user = match serde_json::from_slice(&body) {
        Ok(user) => user,
        Err(err) => return HttpResponse::BadRequest().json(web::Json(Info::new(err.to_string()))),
    };

    match update(user) {
        Some(user) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(user),
        None => HttpResponse::InternalServerError()
            .json(web::Json(Info::new(String::from("Error updating user")))),
    }
}
