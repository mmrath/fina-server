use chrono::DateTime;
use chrono::Utc;
use core::User;
use diesel::insert_into;
use diesel::prelude::*;
use error::{DataError, DataErrorKind};
use failure::ResultExt;
use schema::core::user_password;
use util::db::Connection;

#[derive(
    Queryable, Identifiable, AsChangeset, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[table_name = "user_password"]
#[belongs_to(User)]
#[primary_key(user_id)]
pub struct UserPassword {
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub hash: String,
    pub expiry_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[table_name = "user_password"]
pub struct NewUserPassword {
    pub user_id: i64,
    pub hash: String,
    pub expiry_date: DateTime<Utc>,
}

impl UserPassword {
    pub fn find(conn: &Connection, user_id: i64) -> Result<Option<UserPassword>, DataError> {
        debug!("Finding password by user id {}", user_id);
        let res = user_password::table
            .find(user_id)
            .first(conn)
            .optional()?;
        Ok(res)
    }

    pub fn insert(conn: &Connection, new_password: &NewUserPassword) -> Result<(), DataError> {
        insert_into(user_password::table)
            .values(new_password)
            .execute(conn)?;
        Ok(())
    }
}
