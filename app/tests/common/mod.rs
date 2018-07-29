use fina_util::db::Connection;
use parking_lot::Mutex;

static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! run_test {
    (| $client:ident, $conn:ident | $block:expr) => {{
        ::std::env::set_var(
            "DATABASE_URL",
            "postgres://billac:billac@localhost/billacdb",
        );
        let _lock = DB_LOCK.lock();
        let (rocket, db) = fina_app_lib::rocket();
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");
        $block
    }};
}

#[cfg(test)]
pub fn clean_db(conn: &Connection) {
    use diesel::sql_query;
    use diesel::RunQueryDsl;

    sql_query("TRUNCATE TABLE onetime_token CASCADE")
        .execute(conn)
        .unwrap();
    sql_query("TRUNCATE TABLE user_password CASCADE")
        .execute(conn)
        .unwrap();
    sql_query("TRUNCATE TABLE app_user CASCADE")
        .execute(conn)
        .unwrap();
}

#[cfg(test)]
pub fn truncate_table(_conn: &Connection, _tables: Vec<String>) {}
