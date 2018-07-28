use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;

use failure::{Fail, ResultExt};
use crate::error::Error;

pub type Connection = PgConnection;
pub type ConnectionPool = Pool<ConnectionManager<Connection>>;
pub type PooledConnection = ::r2d2::PooledConnection<ConnectionManager<Connection>>;


/// Creates the database connection pool
pub fn establish_connection_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be specified");
    let manager = ConnectionManager::<Connection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

/*
fn establish_connection() -> DbConnection {
    use diesel::Connection as conn;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be specified");
    DbConnection::establish(&database_url).expect("Unable to create DB connection")
}

*/

pub fn tx<T: Sized, E: Fail + Error, F: FnOnce(&Connection) -> Result<T, E>>(
    conn: &Connection,
    f: F,
) -> Result<T, E> {
    use diesel::connection::TransactionManager;
    use diesel::Connection;

    let tm = conn.transaction_manager();
    let _ = tm.begin_transaction(conn).map_err(E::to_internal_err)?;
    let res = f(conn);

    match res {
        Err(ref e) => if e.is_internal_err() {
            tm.rollback_transaction(conn).map_err(E::to_internal_err)?;
        } else {
            tm.commit_transaction(conn).map_err(E::to_internal_err)?;
        },
        Ok(_) => tm.commit_transaction(conn).map_err(E::to_internal_err)?,
    }

    return res;
}


