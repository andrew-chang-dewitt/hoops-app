use crate::error_template::{AppError, ErrorTemplate};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rust_decimal::prelude::*;
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
    }
}

/// Data type for modeling a transaction's information
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
        use sqlx::{ FromRow, SqlitePool };

        pub fn pool() -> Result<SqlitePool, ServerFnError> {
            // FIXME: so why isn't it showing up in context here?
            use_context::<SqlitePool>()
                .ok_or_else(|| ServerFnError::ServerError("Pool missing".into()))
        }

        #[derive(FromRow, Clone)]
        pub struct TransactionSql {
            id: Uuid,
            payee: String,
            description: Option<String>,
            amount: String,
        }

        impl Into<Transaction> for TransactionSql {
            fn into(self) -> Transaction {
                let TransactionSql { id, amount, description, payee } = self;

                Transaction {
                    id, payee, description,
                    amount: Decimal::from_str_exact(&amount).unwrap(),
                }
            }
        }

        impl Into<TransactionSql> for Transaction {
            fn into(self) -> TransactionSql {
                let Transaction { id, amount, description, payee } = self;

                TransactionSql {
                    id, description, payee,
                    amount: amount.to_string(),
                }
            }
        }

        impl Transaction {
            pub async fn db_insert(self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
                let TransactionSql {id, payee, description, amount} = self.into();

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
    let pool = &pool().map_err(|err| {
        logging::log!("There was an error getting a sqlite pool: {}", err);
        err
    })?;

    Transaction::new(amount, payee, desc_option)
        .db_insert(pool)
        .await
        .map_err(|err| {
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
