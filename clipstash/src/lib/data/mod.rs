use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use std::str::FromStr;
use uuid::Uuid;

pub mod model;
pub mod query;

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

#[derive(Debug, Clone, Display, From, Serialize, Deserialize)]
pub struct DbId(Uuid);

impl DbId {
	pub fn new() -> Self {
		Uuid::new_v4().into()
	}

	pub fn nil() -> Self {
		Self(Uuid::nil())
	}
}

impl Default for DbId {
	fn default() -> Self {
		Self::new()
	}
}

impl From<DbId> for String {
	fn from(id: DbId) -> Self {
		format!("{}", id.0)
	}
}

impl FromStr for DbId {
	type Err = uuid::Error;
	fn from_str(id: &str) -> Result<Self, Self::Err> {
		Ok(Self(Uuid::parse_str(id)?))
	}
}

#[derive(Debug, thiserror::Error)]
pub enum DataError {
	#[error("Database Error: {0}")]
	Database(#[from] sqlx::Error),
}

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
	pub async fn new(connection_str: &str) -> Self {
		let pool = sqlx::sqlite::SqlitePoolOptions::new()
			.connect(connection_str)
			.await;

		match pool {
			Ok(pool) => Self(pool),
			Err(e) => {
				eprintln!("{}\n", e);
				eprintln!(
					"If the database has not yet been created, run:\n    $ sqlx database setup\n"
				);
				panic!("Database Connection Error");
			}
		}
	}

	pub fn get_pool(&self) -> &DatabasePool {
		&self.0
	}
}