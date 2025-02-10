use crate::model::user::dao;
use crate::{controller::authorization_check::verify_token, model::misc::Info};
use actix_web::{http::header::ContentType, post, web, HttpRequest, HttpResponse};
use futures::StreamExt;

#[post("/users")]
pub async fn add(request: HttpRequest, mut payload: web::Payload) -> HttpResponse {
    verify_token(request.headers());

    const MAX_SIZE: usize = 262_144; // max payload size is 256k
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                if body.len() + chunk.len() > MAX_SIZE {
                    return HttpResponse::BadRequest()
                        .json(web::Json(Info::new(String::from("Request body too large"))));
                }
                body.extend_from_slice(&chunk)
            }
            Err(err) => {
                return HttpResponse::NoContent().json(web::Json(Info::new(err.to_string())))
            }
        }
    }

    let user = match serde_json::from_slice(&body) {
        Ok(u) => u,
        Err(err) => return HttpResponse::BadRequest().json(web::Json(Info::new(err.to_string()))),
    };

    let user = dao::add(user);
    match user {
        Some(u) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(u),
        None => HttpResponse::InternalServerError().json(web::Json(Info::new(String::from(
            "Error inserting new user to database",
        )))),
    }

    // let msg = web::Json(Info{
    //     message: String::from("test message")
    // });
}
