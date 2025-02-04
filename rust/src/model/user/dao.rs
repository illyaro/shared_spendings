use crate::model::database_connection::databse_connetion::establish_connection;
use crate::model::user::user::User;
use diesel::prelude::*;

pub fn get_all() -> Vec<User> {
    use crate::model::schema::username::dsl::*;

    let connection = &mut establish_connection();

    username
        .select(User::as_select())
        .load(connection)
        .expect("Error getting all users")
}

pub fn get_one(id: String) -> Option<User> {
    use crate::model::schema::username::dsl::username;

    let connection = &mut establish_connection();
    let user = username
        .find(id)
        .select(User::as_select())
        .first(connection)
        .optional();

    match user {
        Ok(Some(u)) => Some(u),
        Ok(None) => None,
        Err(_) => {
            println!("Error getting user with this ID");
            None
        }
    }
}

pub fn add(user: User) -> Option<User> {
    use crate::model::schema::username;

    let connection = &mut establish_connection();

    let res = diesel::insert_into(username::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(connection);

    match res {
        Ok(user) => Some(user),
        Err(_) => {
            println!("Error inserting user");
            None
        }
    }
}

pub fn update(user: User) -> Option<User> {
    use crate::model::schema::username::dsl::*;

    let connection = &mut establish_connection();

    let ret = diesel::update(username.find(user.get_id()))
        .set((
            email.eq(user.get_email()),
            picture.eq(user.get_photo()),
            user_name.eq(user.get_name()),
        ))
        .returning(User::as_returning())
        .get_result(connection);

    match ret {
        Ok(u) => Some(u),
        Err(_) => {
            println!("Could not update user");
            None
        }
    }
}

pub fn delete(user_id: String) -> usize {
    use crate::model::schema::username::dsl::*;

    let connection = &mut establish_connection();

    diesel::delete(username.filter(id.eq(user_id)))
        .execute(connection)
        .expect("Error deleting user")
}
