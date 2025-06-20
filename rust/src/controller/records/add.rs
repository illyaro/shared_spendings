use crate::{
    controller::authorization_check::verify_token,
    model::{
        misc::Info,
        record::{dao, record::NewRecord},
    },
};
use actix_web::{http::header::ContentType, post, web, HttpRequest, HttpResponse};
use futures::StreamExt;

#[post("/records")]
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
    // println!("Before parsing new record");
    let record = match serde_json::from_slice::<NewRecord>(&body) {
        Ok(record) => record,
        Err(err) => return HttpResponse::BadRequest().json(web::Json(Info::new(err.to_string()))),
    };

    // println!("about to add record: {:?}", &record);

    // let dt = chrono::Local::now().naive_local();
    let user = dao::add(record);

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(user)
}
