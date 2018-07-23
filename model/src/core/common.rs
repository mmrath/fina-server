use diesel::prelude::*;

use error::{DataError, DataErrorKind};
use failure::ResultExt;
use schema::core::*;
use util::db::Connection;

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "date_format"]
pub struct DateFormat {
    pub id: i32,
    pub c_format: String,
    pub date_picker_format: String,
    pub js_format: String,
}

impl DateFormat {
    pub fn find_all(conn: &Connection) -> Result<Vec<DateFormat>, DataError> {
        debug!("Fetching date formats");
        let res = date_format::table
            .load(conn)
            .context(DataErrorKind::Internal)?;
        Ok(res)
    }
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "datetime_format"]
pub struct DatetimeFormat {
    pub id: i32,
    pub c_format: String,
    pub js_format: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "timezone"]
pub struct Timezone {
    pub id: i32,
    pub name: String,
    pub gmt_offset: String,
    pub location: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "language"]
pub struct Language {
    pub id: i32,
    pub name: String,
    pub locale: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "country"]
pub struct Country {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub dial_code: i16,
    pub currency: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "currency"]
pub struct Currency {
    pub id: i16,
    pub code: String,
    pub symbol: String,
    pub name: String,
}
