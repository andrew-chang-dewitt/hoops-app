use leptos::*;
use leptos_router::*;
use rust_decimal::prelude::*;

use crate::components::input::{Input, InputAmount};
use crate::transactions::model::Transaction;

#[cfg(feature = "ssr")]
use crate::transactions::model::{db_insert_new, db_read_many, pool};

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
/// Currently just submits the form w/out any error handling or optimistic updating of the list
///
/// TODO:
///
/// 1. error handling
/// 2. optimistic updates to a co-located list
/// 3. add date/time picker for timestamp field
/// 4. build account feature & add account_id as foreign key
/// 5. build envelope feature & add spent_from as nullable foreign key
#[component]
pub fn TransactionNew() -> impl IntoView {
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
/// 4. sort by columns
/// 5. filter by columns
#[component]
pub fn TransactionsAll() -> impl IntoView {
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
