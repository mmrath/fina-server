use chrono::DateTime;
use chrono::Utc;
use diesel::{insert_into, update};
use diesel::prelude::*;
use failure::ResultExt;
use schema::core::app_user;
use util::DbConnection;
use error::{DbError, DbErrorKind};

#[derive(
Queryable, Identifiable, AsChangeset, Associations, Debug, Serialize, Deserialize, Clone,
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

pub enum UserUniqueKey<'a> {
    Username(&'a str),
    Email(&'a str),
    PhoneNumber(&'a str),
}

impl User {
    pub fn insert(conn: &DbConnection, new_user: &NewUser) -> Result<User, DbError> {
        let res = insert_into(app_user::table)
            .values(new_user)
            .get_result(conn)
            .context(DbErrorKind::Internal)?;

        Ok(res)
    }

    pub fn activate(conn: &DbConnection, user: &User) -> Result<(), DbError> {
        use schema::core::app_user::dsl::*;

        let uc = update(app_user)
            .filter(id.eq(user.id))
            .set(activated.eq(true))
            .execute(conn)
            .context(DbErrorKind::Internal)?;
        if uc != 1 {
            Err(DbErrorKind::IncorrectResultSize(1, uc))?
        } else {
            Ok(())
        }
    }
    pub fn update(conn: &DbConnection, user: &User) -> Result<User, DbError> {
        let res = update(app_user::table)
            .set(user)
            .get_result(conn)
            .context(DbErrorKind::Internal)?;
        Ok(res)
    }

    pub fn find(conn: &DbConnection, id: i64) -> Result<User, DbError> {
        debug!("Finding user by id {}", id);
        let res = app_user::table
            .find(id)
            .first(conn)
            .context(DbErrorKind::Internal)?;
        Ok(res)
    }

    pub fn exists_by_username(conn: &DbConnection, uname: &str) -> Result<bool, DbError> {
        use diesel::expression::dsl::exists;
        use diesel::select;
        use schema::core::app_user::dsl::*;

        let res = select(exists(app_user.filter(username.eq(uname))))
            .get_result(conn)
            .context(DbErrorKind::Internal)?;
        Ok(res)
    }

    pub fn find_by_username(conn: &DbConnection, username: &str) -> Result<Option<User>, DbError> {
        use schema::core::app_user;

        let res = app_user::table
            .filter(app_user::username.eq(username))
            .select(app_user::all_columns)
            .first::<User>(conn)
            .optional()
            .context(DbErrorKind::Internal)?;
        Ok(res)
    }
}
