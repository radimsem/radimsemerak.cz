use std::env;

use anyhow::{Ok, Result};
use diesel::{PgConnection, Connection};

use crate::services::pw::complete_db_uri;

pub struct Database {
    pub conn: PgConnection
}

impl Database {
    pub fn build() -> Result<Self> {
        let db_uri = complete_db_uri(&mut env::var("DB_URI")?, env::var("DB_PASSWORD")?)?;
        let conn = PgConnection::establish(&db_uri)?;

        Ok(Self { conn })
    }
}
