use diesel::prelude::*;
use failure::Error;
use schema::core::*;
use util::DbConnection;

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone)]
#[table_name = "date_format"]
pub struct DateFormat {
    pub id: i32,
    pub c_format: String,
    pub date_picker_format: String,
    pub js_format: String,
}

impl DateFormat {
    pub fn find_all(conn: &DbConnection) -> Result<Vec<DateFormat>, Error> {
        debug!("Fetching date formats");
        date_format::table
            .load(conn)
            .map_err(|_err| format_err!("Failed to fetch date formats"))
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
