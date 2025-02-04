use actix_web::{get, HttpResponse, http::header::ContentType};
use crate::model::record::dao::get_all;

#[get("/records")]
pub async fn get() -> HttpResponse{
    let records = get_all();
    
    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(records)
}