use crate::model::record::dao::{get_all, get_all_of_user_and_date_interval};
use actix_web::{get, http::header::ContentType, web, HttpResponse};
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<String>,
    dt_from: Option<NaiveDateTime>,
    dt_to: Option<NaiveDateTime>,
}

#[get("/records")]
pub async fn get(path: web::Query<QueryParams>) -> HttpResponse {
    let query_params = path.into_inner();

    let records =
        if let (Some(user_id), Some(dt_from)) = (query_params.user_id, query_params.dt_from) {
            let dt_to = query_params
                .dt_to
                .unwrap_or_else(|| chrono::Local::now().naive_local());

            get_all_of_user_and_date_interval(user_id, dt_from, dt_to)
        } else {
            get_all()
        };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(records)
}
