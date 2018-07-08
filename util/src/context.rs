use super::db::{DbConnection, PooledDbConnection};

pub struct Context {
    pool: PooledDbConnection,
}

impl Context {
    pub fn new(pc: PooledDbConnection) -> Self {
        Self { pool: pc }
    }

    pub fn db(&self) -> &DbConnection {
        &*self.pool
    }
}
