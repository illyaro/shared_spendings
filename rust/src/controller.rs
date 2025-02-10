pub mod records;
pub mod users;

pub mod authorization_check {
    use actix_web::http::header::HeaderMap;

    pub fn verify_token(headers: &HeaderMap){
        println!("Headers: {:?}", headers);
    }
}