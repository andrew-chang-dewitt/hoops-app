use cfg_if::cfg_if;
use leptos::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data type for modeling a transaction's information
#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub payee: String,
    pub description: Option<String>,
    pub amount: Decimal,
}

impl Transaction {
    pub fn new(amount: Decimal, payee: String, description: Option<String>) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            amount,
            description,
            payee,
        }
    }
}

cfg_if! {
    if #[cfg(feature ="ssr")] {
        use std::convert::TryInto;
        use sqlx::{ FromRow, SqlitePool };

        pub fn pool() -> Result<SqlitePool, ServerFnError> {
            // FIXME: so why isn't it showing up in context here?
            use_context::<SqlitePool>()
                .ok_or_else(|| ServerFnError::ServerError("Pool missing".into()))
        }

        #[derive(FromRow, Clone)]
        pub struct TransactionSql {
            id: String,
            amount: String,
            description: Option<String>,
            payee: String,
        }

        impl TryInto<Transaction> for TransactionSql {
            type Error = anyhow::Error;

            fn try_into(self) -> Result<Transaction, Self::Error> {
                let TransactionSql { id, amount, description, payee } = self;
                // either of these conversions can fail, return early if one does
                let id = Uuid::parse_str(&id)?;
                let amount = Decimal::from_str_exact(&amount)?;

                let tran = Transaction { id, amount, description, payee };

                Ok(tran)
            }
        }

        impl Into<TransactionSql> for Transaction {
            fn into(self) -> TransactionSql {
                let Transaction { id, amount, description, payee } = self;
                let amount = amount.to_string();
                let id = id
                    .hyphenated()
                    .encode_lower(&mut Uuid::encode_buffer())
                    .to_string();

                TransactionSql { id, amount, description, payee }
            }
        }

        pub async fn db_insert_new(transaction: Transaction, pool: &SqlitePool) -> Result<(), sqlx::Error> {
            let TransactionSql {id, payee, description, amount} = transaction.into();

            sqlx::query!(
                r#"
                INSERT INTO transactions (id, amount, description, payee)
                VALUES (?, ?, ?, ?);
                "#,
                id,
                amount,
                description,
                payee,
            )
                .execute(pool)
                .await
                .map(|_| ())
        }

        pub async fn db_read_many(pool: &SqlitePool) -> Result<Vec<Transaction>, anyhow::Error> {
            // needed to enable try_next on returned rows stream
            use futures::TryStreamExt;

            let mut transactions: Vec<Transaction> = Vec::new();
            // rows must be mutable here...
            let mut rows = sqlx::query_as::<_, TransactionSql>(
                r#"
                SELECT * FROM transactions;
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
