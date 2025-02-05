pub mod database_connection;
pub mod record;
pub mod schema;
pub mod user;

pub mod misc {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct Info {
        message: String,
    }

    impl Info {
        pub fn new(message: String) -> Self {
            Self {
                message
            }
        }
    }
}
