use super::client_pg::{PgPool, PgPooledConnection};

pub struct PgContext {
    pub conn: PgPooledConnection,
}

impl PgContext {
    pub fn new(conn: PgPooledConnection) -> PgContext {
        PgContext { conn }
    }
}
