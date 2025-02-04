pub mod databse_connetion {
    use diesel::prelude::*;
    use dotenvy::dotenv;
    use std::env;

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Database url must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
    }
}
