use actix_web::{App, HttpServer};
use shared_spendings::controller::{records, users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // records
            .service(records::get::get)
            .service(records::add::add)
            .service(records::edit::edit)
            .service(records::remove::remove)
            // users
            .service(users::get::get)
            .service(users::add::add)
            .service(users::edit::edit)
            .service(users::remove::remove)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
