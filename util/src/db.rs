use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type DbConnection = PgConnection;
pub type DbConnectionPool = Pool<ConnectionManager<DbConnection>>;
pub type PooledDbConnection = ::r2d2::PooledConnection<ConnectionManager<DbConnection>>;

/// Creates the database connection pool
pub fn establish_connection_pool() -> DbConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be specified");
    let manager = ConnectionManager::<DbConnection>::new(database_url);

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
