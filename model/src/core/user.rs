use chrono::DateTime;
use chrono::Utc;
use crate::error::{DataError, DataErrorKind};
use crate::schema::core::app_user;
use diesel::prelude::*;
use diesel::{insert_into, update};
use failure::ResultExt;
use fina_util::db::Connection;

#[derive(
    Queryable,
    Identifiable,
    AsChangeset,
    Associations,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Eq,
    PartialEq,
)]
#[table_name = "app_user"]
pub struct User {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub activated: bool,
    pub locked: bool,
    pub failed_logins: i16,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "app_user"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSignUp {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

pub enum UserUniqueKey<'a> {
    Username(&'a str),
    Email(&'a str),
    PhoneNumber(&'a str),
}

impl User {
    pub fn insert(conn: &Connection, new_user: &NewUser<'_>) -> Result<User, DataError> {
        let res = insert_into(app_user::table)
            .values(new_user)
            .get_result(conn)?;

        Ok(res)
    }

    pub fn activate(conn: &Connection, user: &User) -> Result<(), DataError> {
        use crate::schema::core::app_user::dsl::*;

        let uc = update(app_user)
            .filter(id.eq(user.id))
            .set(activated.eq(true))
            .execute(conn)?;
        if uc != 1 {
            Err(DataErrorKind::IncorrectResultSize(1, uc))?
        } else {
            Ok(())
        }
    }
    pub fn update(conn: &Connection, user: &User) -> Result<User, DataError> {
        let res = update(app_user::table).set(user).get_result(conn)?;
        Ok(res)
    }

    pub fn find(conn: &Connection, id: i64) -> Result<User, DataError> {
        debug!("Finding user by id {}", id);
        let res = app_user::table.find(id).first(conn)?;
        Ok(res)
    }

    pub fn exists_by_username(conn: &Connection, uname: &str) -> Result<bool, DataError> {
        use crate::schema::core::app_user::dsl::*;
        use diesel::expression::dsl::exists;
        use diesel::select;

        let res = select(exists(app_user.filter(username.eq(uname)))).get_result(conn)?;
        Ok(res)
    }

    pub fn find_by_username(conn: &Connection, username: &str) -> Result<Option<User>, DataError> {
        use crate::schema::core::app_user;

        let res = app_user::table
            .filter(app_user::username.eq(username))
            .select(app_user::all_columns)
            .first::<User>(conn)
            .optional()?;
        Ok(res)
    }
}
