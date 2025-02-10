use crate::{
    controller::authorization_check::verify_token,
    model::{
        misc::Info,
        record::{dao, record::Record},
    },
};
use actix_web::{http::header::ContentType, put, web, HttpRequest, HttpResponse};
use futures::StreamExt;

#[put("/records")]
pub async fn edit(request: HttpRequest, mut payload: web::Payload) -> HttpResponse {
    verify_token(request.headers());

    const MAX_SIZE: usize = 262_144; //256KB
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                if body.len() + chunk.len() > MAX_SIZE {
                    return HttpResponse::BadRequest()
                        .json(web::Json(Info::new(String::from("Body size is too large"))));
                }
                body.extend_from_slice(&chunk);
            }
            Err(err) => {
                return HttpResponse::BadRequest().json(web::Json(Info::new(err.to_string())))
            }
        }
    }

    let record = match serde_json::from_slice::<Record>(&body) {
        Ok(record) => record,
        Err(err) => return HttpResponse::BadRequest().json(web::Json(Info::new(err.to_string()))),
    };

    let record = dao::edit(record);

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(record)
}
