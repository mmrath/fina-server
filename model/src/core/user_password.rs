use chrono::DateTime;
use chrono::Utc;
use core::User;
use diesel::insert_into;
use diesel::prelude::*;
use failure::Error;
use schema::core::user_password;
use util::DbConnection;

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
    pub fn find(conn: &DbConnection, user_id: i64) -> Result<UserPassword, Error> {
        debug!("Finding password by user id {}", user_id);
        user_password::table
            .find(user_id)
            .first(conn)
            .map_err(|_err| format_err!("Failed to fetch password by id {}", user_id))
    }

    pub fn insert(conn: &DbConnection, new_password: &NewUserPassword) -> Result<(), Error> {
        insert_into(user_password::table)
            .values(new_password)
            .execute(conn)
            .map_err(|_err| format_err!("Failed to create password"))?;
        Ok(())
    }
}
