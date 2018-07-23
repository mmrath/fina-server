use super::db::{Connection, PooledConnection};

pub struct Context {
    pool: PooledConnection,
}

impl Context {
    pub fn new(pc: PooledConnection) -> Self {
        Self { pool: pc }
    }

    pub fn db(&self) -> &Connection {
        &*self.pool
    }
}
