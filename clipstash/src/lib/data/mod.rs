//! Database models and queries.
pub mod model;
pub mod query;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;
use sqlx::Sqlite;

/// The possible errors that may occur when working with a database.
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    /// Database error.
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error)
}

/// Concrete database pool wrapper.
pub type AppDatabase = Database<Sqlite>;
/// Concrete database pool.
pub type DatabasePool = sqlx::sqlite::SqlitePool;
/// Concrete database transaction.
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
/// Concrete database row.
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
/// Concrete database query result.
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

/// Wrapper around a database pool.
pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

/// Implementation of the `Database` wrapper for `Sqlite`.
impl Database<Sqlite> {
    /// Create a new `Database` with the provided `connection_string`.
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!("If the database has not yet been created, run: \n   $ sqlx database setup\n");
                panic!("database connection error");
            }
        }
    }
    /// Get a reference to the database connection pool.
    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

/// Internal database ID that can be used for any ID purposes.
#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    /// Create a new database ID.
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    /// Create an empty database ID.
    ///
    /// This database ID is always the same. It can be used to obscure an
    /// actual ID when working with clients.
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

/// The default behavior is to create a [`DbId`]
impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}


impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}

#[cfg(test)]
pub mod test {
    use crate::data::*;
    use tokio::runtime::Handle;

    pub fn new_db(handle: &Handle) -> AppDatabase {
        use sqlx::migrate::Migrator;
        use std::path::Path;
        handle.block_on(async move {
            let db = Database::new(":memory:").await;
            let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
            let pool = db.get_pool();
            migrator.run(pool).await.unwrap();
            db
        })
    }
}