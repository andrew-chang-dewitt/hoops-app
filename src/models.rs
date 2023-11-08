#![cfg(feature = "ssr")]

use sqlx::{sqlite::SqliteRow, FromRow, SqlitePool};

/// Declare a struct to have a specific table name
///
/// ```
/// use hoops_app::models::Table;
///
/// struct MyStruct();
///
/// impl<'a> Table<'a> for MyStruct {
///   const TABLE: &'a str = "my_table_name";
/// }
///
/// assert_eq!(MyStruct::TABLE, "my_table_name");
/// ```
pub trait Table {
    const TABLE: &'static str;
}

/// Methods for saving a type to a database.
///
/// Requires a second type that the first one can be translated into/from for the actual data types
/// that will be stored in the database. Also requires that the type knows it's table name, via
/// the `Table` trait.
///
/// ```
/// use uuid::Uuid;
/// use std::convert::TryInto;
/// use sqlx::{FromRow, SqlitePool, sqlite::SqliteRow};
///
/// use hoops_app::models::{Table, Create};
///
/// struct MyType {
///   id: Uuid,
///   num: usize,
/// }
///
/// #[derive(FromRow, Clone)]
/// struct MyTypeSql {
///   id: String,
///   num: usize,
/// }
///
/// impl Into<MyTypeSql> for MyType {
///   fn into(self) -> MyTypeSql {
///     let Self { id, num } = self;
///     let id = id
///         .hyphenated()
///         .encode_lower(&mut Uuid::encode_buffer())
///         .to_string();
///     
///     MyTypeSql { id, num }
///   }
/// }
///
/// impl TryInto<MyType> for MyTypeSql {
///     type Error = anyhow::Error;
///
///   fn try_into(self) -> Result<MyType, Self::Error> {
///     let Self { id, num } = self;
///     // either of these conversions can fail, return early if one does
///     let id = Uuid::parse_str(&id)?;
///
///     Ok(MyType {id, num})
///   }
/// }
///
/// impl Table for MyType {
///   const TABLE: &'static str = "table_name";
/// }
///
/// impl Create for MyType {
///   async fn create_one(self, pool: SqlitePool) -> Result<(), anyhow::Error> {
///     todo!()
///   }
///
///   async fn create_many(pool: SqlitePool, items: Vec<Self>) -> Result<usize, anyhow::Error> {
///     todo!()
///   }
/// }
/// ```
pub trait Create<'r, SqlType>
where
    Self: Into<SqlType>,
    SqlType: TryInto<Self> + FromRow<'r, SqliteRow>,
{
    /// Insert the given item into the database
    fn create_one(
        self,
        pool: SqlitePool,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send;
    /// Insert the given items into the database
    fn create_many(
        pool: SqlitePool,
        items: Vec<Self>,
    ) -> impl std::future::Future<Output = Result<usize, anyhow::Error>> + Send;
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

pub trait Read<'r, SqlType>
where
    Self: Into<SqlType>,
    SqlType: TryInto<Self> + FromRow<'r, SqliteRow>,
{
    /// Read one item with given ID from the database, if it exists
    fn read_one_by_id<Id>(
        self,
        pool: SqlitePool,
        id: Id,
    ) -> impl std::future::Future<Output = Result<Option<Self>, anyhow::Error>> + Send;
    /// Read many items from the database
    fn read_many(
        self,
        pool: SqlitePool,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, anyhow::Error>> + Send;
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
