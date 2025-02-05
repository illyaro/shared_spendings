use actix_web::{http::header::ContentType, web, HttpResponse, post};
use futures::StreamExt;
use crate::model::{misc::Info, record::{dao, record::NewRecord}};

#[post("/records/add")]
pub async fn add(mut payload: web::Payload) -> HttpResponse {
    const MAX_SIZE: usize = 262_144; // max payload size is 256k
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                if body.len() + chunk.len() > MAX_SIZE {
                    return HttpResponse::BadRequest().json(web::Json(Info::new(String::from("Request body too large"))));
                }
                body.extend_from_slice(&chunk)
            }
            Err(err) => return HttpResponse::NoContent().json(web::Json(Info::new(err.to_string())))
        }
    }

    let record = match serde_json::from_slice::<NewRecord>(&body) {
        Ok(record) => record,
        Err(err) => return HttpResponse::BadRequest().json(web::Json(Info::new(err.to_string()))),
    };

    let dt = chrono::Local::now().naive_local();
    let user = dao::add(record, dt);

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(user)
}