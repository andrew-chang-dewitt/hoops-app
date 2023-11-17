#![cfg(feature = "ssr")]

use sqlx::{sqlite::SqliteRow, FromRow, SqlitePool};

/// Declare a struct to have a specific table name
///
/// ```
/// use hoops_app::models::Table;
///
/// struct MyStruct();
///
/// impl Table for MyStruct {
///   const TABLE: &'static str = "my_table_name";
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
/// use std::convert::TryFrom;
/// use sqlx::{FromRow, SqlitePool, sqlite::SqliteRow};
///
/// use hoops_app::models::{ Create, Table };
///
/// struct MyType {
///   id: Uuid,
///   txt: String,
/// }
///
/// #[derive(FromRow, Clone)]
/// struct MyTypeSql {
///   id: String,
///   txt: String,
/// }
///
/// impl From<MyType> for MyTypeSql {
///   fn from(value: MyType) -> Self {
///     let MyType { id, txt } = value;
///     let id = id
///         .hyphenated()
///         .encode_lower(&mut Uuid::encode_buffer())
///         .to_string();
///
///     Self { id, txt }
///   }
/// }
///
/// impl TryFrom<MyTypeSql> for MyType {
///   type Error = anyhow::Error;
///
///   fn try_from(value: MyTypeSql) -> Result<Self, Self::Error> {
///     let MyTypeSql { id, txt } = value;
///     let id = Uuid::parse_str(&id)?;
///
///     Ok(MyType{ id, txt })
///   }
/// }
///
/// // save the associated table name as a const on MyType
/// impl Table for MyType {
///   const TABLE: &'static str = "my_table";
/// }
///
/// // implement default create methods for MyType
/// impl Create<'_> for MyType {
///   type SqlType = MyTypeSql;
/// }
///
/// // create a new MyType value using the newly implemented default methods
/// MyType::create_one(MyType {
///   id: Uuid::new_v4(),
///   txt: String::from("Some text"),
/// });
///
/// // TODO: assert it exists? probably just use sqlx directly for that...
/// ```
pub trait Create<'r>: Sized + Table {
    type SqlType: From<Self> + FromRow<'r, SqliteRow>;

    /// Insert the given item into the database
    fn create_one(
        pool: &SqlitePool,
        value: Self,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send {
        // TODO: impl this, prob w/ sqlx::QueryBuilder???
        unimplemented!();
    }
}

/// Methods for saving a type to a database.
///
/// Requires a second type that the first one can be translated into/from for the actual data types
/// that will be stored in the database. Also requires that the type knows it's table name, via
/// the `Table` trait.
///
/// ```
/// use uuid::Uuid;
/// use std::convert::TryFrom;
/// use sqlx::{FromRow, SqlitePool, sqlite::SqliteRow};
///
/// use hoops_app::models::{ Read, Table };
///
/// struct MyType {
///   id: Uuid,
///   txt: String,
/// }
///
/// #[derive(FromRow, Clone)]
/// struct MyTypeSql {
///   id: String,
///   txt: String,
/// }
///
/// impl From<MyType> for MyTypeSql {
///   fn from(value: MyType) -> Self {
///     let MyType { id, txt } = value;
///     let id = id
///         .hyphenated()
///         .encode_lower(&mut Uuid::encode_buffer())
///         .to_string();
///
///     Self { id, txt }
///   }
/// }
///
/// impl TryFrom<MyTypeSql> for MyType {
///   type Error = anyhow::Error;
///
///   fn try_from(value: MyTypeSql) -> Result<Self, Self::Error> {
///     let MyTypeSql { id, txt } = value;
///     let id = Uuid::parse_str(&id)?;
///
///     Ok(MyType{ id, txt })
///   }
/// }
///
/// impl Table for MyType {
///   const TABLE: &'static str = "my_table";
/// }
///
/// impl Read<'_> for MyType {
///   type SqlType = MyTypeSql;
///
///   async fn read_many(self, pool: &SqlitePool) -> Result<Vec<MyType>, anyhow::Error> {
///     // write actual sqlx query here?
///     // ideally I can do a default impl for this one since it'll be so common...
///     todo!()
///   }
/// }
/// ```
pub trait Read<'r>: TryFrom<Self::SqlType> + Table + Sized {
    type SqlType: From<Self> + FromRow<'r, SqliteRow>;

    /// Read one item with given ID from the database, if it exists
    //fn read_one_by_id<Id>(
    //    self,
    //    pool: SqlitePool,
    //    id: Id,
    //) -> impl std::future::Future<Output = Result<Option<Self>, anyhow::Error>> + Send;
    /// Read many items from the database
    fn read_many(
        self,
        pool: &SqlitePool,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, anyhow::Error>> + Send;
}
