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
        use std::convert::TryFrom;
        use sqlx::{ FromRow, SqlitePool };

        use crate::models::{Create, Table};

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

        impl TryFrom<TransactionSql> for Transaction {
            type Error = anyhow::Error;

            fn try_from(value: TransactionSql) -> Result<Self, Self::Error> {
                let TransactionSql { id, amount, description, payee, timestamp } = value;
                // any of these conversions can fail, return early if one does
                let id = Uuid::parse_str(&id)?;
                let amount = Decimal::from_str_exact(&amount)?;
                let timestamp = DateTime::from(DateTime::parse_from_rfc3339(&timestamp)?);

                Ok(Transaction { id, amount, description, payee, timestamp })
            }
        }

        impl From<Transaction> for TransactionSql {
            fn from(value: Transaction) -> Self {
                let Transaction { id, amount, description, payee, timestamp } = value;
                let amount = amount.to_string();
                let id = id
                    .hyphenated()
                    .encode_lower(&mut Uuid::encode_buffer())
                    .to_string();
                let timestamp = timestamp.to_rfc3339();

                Self { id, amount, description, payee, timestamp }
            }
        }

        impl Table for Transaction {
            const TABLE: &'static str = "transactions";
        }

        impl Create<'_> for Transaction {
            type SqlType = TransactionSql;

            async fn create_one(pool: &SqlitePool, value: Self) -> Result<(), anyhow::Error> {
                let TransactionSql {id, amount, description, payee, timestamp} = value.into();

                // TODO:
                //
                // I've been looking for a way to generalize this so Create can provide a default
                // impl or an internal default impl that is intended to be used by the implementor
                // to make creating this function easier.
                //
                // Seems like I could get all the struct's field names using macros, but not sure I
                // want to go that route. A super naive version here: https://stackoverflow.com/questions/29986057/is-there-a-way-to-get-the-field-names-of-a-struct-in-a-macro
                // But that just returns a vec of strs, so not exactly useful still?
                //
                // Could also look at a proc macro, but that's a much hairier beast...
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
                    .map_err(|e| e.into())
            }
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
