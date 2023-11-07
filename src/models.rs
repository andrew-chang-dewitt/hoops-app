pub trait Create<SqlType>
where
    Self: Into<SqlType>,
    SqlType: TryInto<Self>,
{
    /// Insert the given item into the database
    async fn create_one(self, item: SqlType) -> Self;
    /// Insert the given items into the database
    async fn create_many(self, items: Vec<SqlType>) -> usize;
}

// pub async fn db_insert_new(transaction: Transaction, pool: &SqlitePool) -> Result<(), sqlx::Error> {
//     let TransactionSql {id, amount, description, payee, timestamp} = transaction.into();
//
//     sqlx::query!(
//         r#"
//         INSERT INTO transactions (id, amount, description, payee, timestamp)
//         VALUES (?, ?, ?, ?, ?);
//         "#,
//         id,
//         amount,
//         description,
//         payee,
//         timestamp,
//     )
//         .execute(pool)
//         .await
//         .map(|_| ())
// }

pub trait Read<SqlType>
where
    Self: Into<SqlType>,
    SqlType: TryInto<Self>,
{
    /// Read one item with given ID from the database, if it exists
    async fn read_one_by_id<Id>(self, id: Id) -> Result<Option<Self>, anyhow::Error>;
    /// Read many items from the database
    async fn read_many(self) -> Result<Vec<Self>, anyhow::Error>;
}

// pub async fn db_read_many(pool: &SqlitePool) -> Result<Vec<Transaction>, anyhow::Error> {
//     // needed to enable try_next on returned rows stream
//     use futures::TryStreamExt;
//
//     let mut transactions: Vec<Transaction> = Vec::new();
//     // rows must be mutable here...
//     let mut rows = sqlx::query_as::<_, TransactionSql>(
//         r#"
//         SELECT * FROM transactions
//         ORDER BY timestamp DESC;
//         "#
//     ).fetch(pool);
//
//     // ...because it is destructively consumed here
//     while let Some(row) = rows.try_next().await? {
//         let tran: Transaction = row.try_into()?;
//         transactions.push(tran);
//     }
//
//     Ok(transactions)
// }
