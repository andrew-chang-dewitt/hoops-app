use crate::error_template::{AppError, ErrorTemplate};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/hoops-app.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <TransactionNew />
        <TransactionsAll />
    }
}

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

/// add Transaction server endpoint
#[server(prefix = "/api", endpoint = "transaction/new")]
pub async fn transaction_new(
    description: String,
    payee: String,
    amount: Decimal,
) -> Result<(), ServerFnError> {
    // convert empty strings to None, otherwise pass as Some(..)
    let desc_option = match description.as_str() {
        "" => None,
        _ => Some(description),
    };
    // if getting a pool fails, immediately return the error instead of proceeding
    let pool = &pool()?;

    let transaction = Transaction::new(amount, payee, desc_option);
    db_insert_new(transaction, pool).await.map_err(|err| {
        logging::log!("There was an error saving the transaction: {}", err);
        ServerFnError::ServerError(err.to_string())
    })
}

/// UI for adding a transaction to the record
#[component]
fn TransactionNew() -> impl IntoView {
    let transaction_new = create_server_action::<TransactionNew>();

    view! {
        <ActionForm action=transaction_new>
            <Input name="payee".to_string() label="Payee:".to_string() attr:required=true />
            <Input name="description".to_string() label="Description:".to_string() />
            <InputAmount name="amount".to_string() label="Amount:".to_string() attr:required=true />
            <button type="submit">Create</button>
        </ActionForm>
    }
}

/// Server endpoint for reading all transaction
#[server(prefix = "/api", endpoint = "transactions/read/all")]
pub async fn transactions_read_many() -> Result<Vec<Transaction>, ServerFnError> {
    let pool = &pool()?;

    db_read_many(pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/// UI for displaying a list of transactions
///
/// Currently just displays all transactions in the database. Needs the following features
/// implemented later:
///
/// TODO:
///
/// 1. paginate or infinite scroll
/// 2. auto-update if new transactions are added in the currently visible range of transactions
/// 3. auto-update a transaction's displayed info if it is updated in the db & it is currently
///    visible
#[component]
fn TransactionsAll() -> impl IntoView {
    let transactions = create_resource(
        // FIXME: for now, this just reads once, later, hook it up to a server action's
        // `version.get()` Signal to fetch updates any time there's changes
        || {},
        move |_| transactions_read_many(),
    );

    view! {
        <Suspense fallback=move || view! {<p>Loading...</p>}.into_view()>
            {move || {
                let existing_transactions = {
                    move || {
                        transactions.get().map(move |t| match t {
                            Err(err) => {
                                view! { <pre>Error fetching transactions: {err.to_string()}</pre>}.into_view()
                            },
                            Ok(trans) => {
                                if trans.is_empty() {
                                    view! {<p>No transactions yet...</p>}.into_view()
                                } else {
                                    trans.into_iter().map(move |tran| {
                                        view! {
                                            <li>
                                                <ul>
                                                    <li>{tran.payee}</li>
                                                    <li>{tran.amount.to_string()}</li>
                                                    <li>{tran.description}</li>
                                                </ul>
                                            </li>
                                        }
                                    }).collect_view()
                                }
                            }
                        }).unwrap_or_default()
                    }
                };

                // FIXME: when the resource relies on watching a server action for adding new
                // transaction, get pending submissions for the action from `action.submissions()`
                // like in https://github.com/leptos-rs/leptos/blob/5f53a1459ebc8ac1912df99ce24153c675a198ed/examples/todo_app_sqlite_axum/src/todo.rs#L172
                // let pending_transactions = {...}

                view! {
                    <ul>
                        {existing_transactions}
                    </ul>
                }
            }}
        </Suspense>
    }
}

/// Reusable amount input component
#[component]
pub fn InputAmount(
    name: String,
    label: String,
    #[prop(optional)] value: String,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <Input {..attrs} name label value input_type=InputType::Number attr:step=0.01 attr:min=0.00 />
    }
}

/// Reusable text input component
#[component]
pub fn Input(
    name: String,
    label: String,
    #[prop(default = InputType::Text)] input_type: InputType,
    #[prop(optional)] value: String,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let input_type_str: String = input_type.into();

    view! {
        <label for=&name>{&label}</label>
        <input {..attrs} id=&name name=&name type=&input_type_str value=&value />
    }
}

pub enum InputType {
    Hidden,
    Number,
    Password,
    Text,
}

impl Into<String> for InputType {
    fn into(self) -> String {
        match self {
            InputType::Hidden => String::from("hidden"),
            InputType::Number => String::from("number"),
            InputType::Password => String::from("password"),
            InputType::Text => String::from("text"),
        }
    }
}
