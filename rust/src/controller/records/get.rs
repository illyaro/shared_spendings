use crate::{
    controller::authorization_check::verify_token,
    model::record::dao::get_all_of_user_and_date_interval,
};
use actix_web::{get, http::header::ContentType, web, HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<String>,
    dt_from: Option<NaiveDateTime>,
    dt_to: Option<NaiveDateTime>,
}

#[get("/records")]
pub async fn get(request: HttpRequest, path: web::Query<QueryParams>) -> HttpResponse {
    verify_token(request.headers());

    let mut query_params = path.into_inner();

    match (query_params.dt_from, query_params.dt_to) {
        (Some(_), None) => query_params.dt_to = Some(chrono::Local::now().naive_local()),
        (None, Some(_)) => {
            query_params.dt_from = Some(
                chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            )
        }
        (_, _) => (),
    }

    let records = get_all_of_user_and_date_interval(
        query_params.user_id,
        query_params.dt_from,
        query_params.dt_to,
    );

    // let records =
    //     if let (Some(user_id), Some(dt_from)) = (query_params.user_id, query_params.dt_from) {
    //         let dt_to = query_params
    //             .dt_to
    //             .unwrap_or_else(|| chrono::Local::now().naive_local());

    //         get_all_of_user_and_date_interval(user_id, dt_from, dt_to)
    //     } else {
    //         get_all()
    //     };

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(records)
}
