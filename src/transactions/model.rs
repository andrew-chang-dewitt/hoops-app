use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use leptos::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data type for modeling a transaction's information
#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub amount: Decimal,
    pub description: Option<String>,
    pub payee: String,
    pub timestamp: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        amount: Decimal,
        payee: String,
        timestamp: DateTime<Utc>,
        description: Option<String>,
    ) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            amount,
            description,
            payee,
            timestamp,
        }
    }
}

// TODO:
//
// - [ ] Generalize db_* methods into a collection of Traits to impl a Table<Model, ModelSql> type that can contain
//       the most common CRUD methods needed (e.g. create_one, read_one_by_id, read_many,
//       update_one_by_id, delete_one_by_id). Table struct might look like this:
//       ```
//       struct Table<Model, ModelSql>
//       where
//          Model: Into<ModelSql>,
//          ModelSql: TryInto<Model>,
//      {
//          // used to hold a table name to use when dynamically building queries
//          table_name: String
//      }
//       ```
// - [ ] add ORDER_BY clause w/ default ordering to db_read_many
// - [ ] build account feature & add account_id as foreign key
// - [ ] build envelope feature & add spent_from as nullable foreign key
cfg_if! {
    if #[cfg(feature ="ssr")] {
        use std::convert::TryInto;
        use sqlx::{ FromRow, SqlitePool };

        pub fn pool() -> Result<SqlitePool, ServerFnError> {
            use_context::<SqlitePool>()
                .ok_or_else(|| ServerFnError::ServerError("Pool missing".into()))
        }

        #[derive(FromRow, Clone)]
        pub struct TransactionSql {
            id: String,
            amount: String,
            description: Option<String>,
            payee: String,
            timestamp: String,
        }

        impl TryInto<Transaction> for TransactionSql {
            type Error = anyhow::Error;

            fn try_into(self) -> Result<Transaction, Self::Error> {
                let TransactionSql { id, amount, description, payee, timestamp } = self;
                // either of these conversions can fail, return early if one does
                let id = Uuid::parse_str(&id)?;
                let amount = Decimal::from_str_exact(&amount)?;
                let timestamp = DateTime::from(DateTime::parse_from_rfc3339(&timestamp)?);

                let tran = Transaction { id, amount, description, payee, timestamp };

                Ok(tran)
            }
        }

        impl Into<TransactionSql> for Transaction {
            fn into(self) -> TransactionSql {
                let Transaction { id, amount, description, payee, timestamp } = self;
                let amount = amount.to_string();
                let id = id
                    .hyphenated()
                    .encode_lower(&mut Uuid::encode_buffer())
                    .to_string();
                let timestamp = timestamp.to_rfc3339();

                TransactionSql { id, amount, description, payee, timestamp }
            }
        }

        // TODO:
        // let's refactor these two db methods into traits for creating and reading records in a
        // database. might look something like this:
        //
        // ```
        // pub trait Create<T, S> {
        //   /// Insert the given item into the database
        //   async fn create_one(self, item: S) -> T;
        //   /// Insert the given items into the database
        //   async fn create_many(self, items: Vec<S>) -> usize;
        // }
        // ```
        pub async fn db_insert_new(transaction: Transaction, pool: &SqlitePool) -> Result<(), sqlx::Error> {
            let TransactionSql {id, amount, description, payee, timestamp} = transaction.into();

            sqlx::query!(
                r#"
                INSERT INTO transactions (id, amount, description, payee, timestamp)
                VALUES (?, ?, ?, ?, ?);
                "#,
                id,
                amount,
                description,
                payee,
                timestamp,
            )
                .execute(pool)
                .await
                .map(|_| ())
        }

        // TODO:
        // ```
        // pub trait Read<T, Id> {
        //   /// Read one item with given ID from the database, if it exists
        //   async fn read_one_by_id(self, item: Id) -> Result<Option<T>, anyhow::Error>;
        //   /// Read many items from the database
        //   async fn read_many(self) -> Result<Vec<T>, anyhow::Error>;
        // }
        // ```
        pub async fn db_read_many(pool: &SqlitePool) -> Result<Vec<Transaction>, anyhow::Error> {
            // needed to enable try_next on returned rows stream
            use futures::TryStreamExt;

            let mut transactions: Vec<Transaction> = Vec::new();
            // rows must be mutable here...
            let mut rows = sqlx::query_as::<_, TransactionSql>(
                r#"
                SELECT * FROM transactions
                ORDER BY timestamp DESC;
                "#
            ).fetch(pool);

            // ...because it is destructively consumed here
            while let Some(row) = rows.try_next().await? {
                let tran: Transaction = row.try_into()?;
                transactions.push(tran);
            }

            Ok(transactions)
        }
    }
}
