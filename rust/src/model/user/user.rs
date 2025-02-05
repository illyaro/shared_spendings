use std::fmt::Display;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Default, Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::model::schema::username)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: String,
    email: String,
    picture: String,
    user_name: String,
}

impl User {
    pub fn new(id: String, email: String, picture: String, user_name: String) -> Self {
        Self {
            id,
            email,
            picture,
            user_name,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_photo(&self) -> &str {
        &self.picture
    }

    pub fn get_name(&self) -> &str {
        &self.user_name
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:25}{} with email: {}, ",
            &self.id, &self.user_name, &self.email
        )
    }
}
