use super::User;
use chrono::DateTime;
use chrono::Utc;
use crate::error::{DataError, DataErrorKind};
use crate::schema::core::onetime_token;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::insert_into;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::serialize;
use diesel::serialize::IsNull;
use diesel::serialize::Output;
use diesel::types::ToSql;
use fina_util::db::Connection;
use std::io::Write;

use diesel::sql_types::Varchar;
use failure::ResultExt;

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone)]
#[table_name = "onetime_token"]
#[belongs_to(User)]
pub struct OnetimeToken {
    pub id: i64,
    pub user_id: Option<i64>,
    pub token_type: TokenType,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "onetime_token"]
pub struct NewOnetimeToken {
    pub user_id: Option<i64>,
    pub token_type: TokenType,
    pub token: String,
    pub expiry_date: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSqlRow, AsExpression)]
#[sql_type = "Varchar"]
pub enum TokenType {
    UserActivation,
    PasswordReset,
}

impl ToSql<Varchar, Pg> for TokenType {
    fn to_sql<W: Write>(&self, out: &mut Output<'_, W, Pg>) -> serialize::Result {
        match *self {
            TokenType::UserActivation => out.write_all(b"UserActivation")?,
            TokenType::PasswordReset => out.write_all(b"PasswordReset")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Varchar, Pg> for TokenType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"AccountActivation" => Ok(TokenType::UserActivation),
            b"PasswordReset" => Ok(TokenType::PasswordReset),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl OnetimeToken {
    pub fn find_by_token(
        conn: &Connection,
        token_key: &str,
    ) -> Result<Option<OnetimeToken>, DataError> {
        use crate::schema::core::onetime_token::dsl::*;
        debug!("Finding key {}", token_key);
        let res = onetime_token
            .filter(token.eq(token_key))
            .first(conn)
            .optional()?;

        Ok(res)
    }

    pub fn find_user_and_token(
        conn: &Connection,
        token_key: &str,
    ) -> Result<Option<(OnetimeToken, User)>, DataError> {
        use crate::schema::core::app_user;
        use crate::schema::core::onetime_token;
        debug!("Finding key {}", token_key);
        let res = onetime_token::table
            .filter(onetime_token::token.eq(token_key))
            .inner_join(app_user::table)
            .select((onetime_token::all_columns, app_user::all_columns))
            .first::<(OnetimeToken, User)>(conn)
            .optional()?;
        Ok(res)
    }

    pub fn insert(conn: &Connection, new: &NewOnetimeToken) -> Result<(), DataError> {
        debug!("Creating key {:?}", new);
        insert_into(onetime_token::table).values(new).execute(conn)?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), DataError> {
        use crate::schema::core::onetime_token;
        use diesel::delete;

        debug!("Deleting token with id {:?}", id);
        delete(onetime_token::table)
            .filter(onetime_token::id.eq(id))
            .execute(conn)?;
        Ok(())
    }
}
