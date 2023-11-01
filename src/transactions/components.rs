use leptos::*;
use leptos_router::*;
use rust_decimal::prelude::*;

use crate::components::input::{Input, InputAmount};
use crate::transactions::model::Transaction;

#[cfg(feature = "ssr")]
use crate::transactions::model::{db_insert_new, db_read_many, pool};

/// UI for adding a transaction to the record
/// Currently just submits the form w/out any error handling or optimistic updating of the list
///
/// TODO:
///
/// - [ ] error handling
/// - [x] optimistic updates to a co-located list
/// - [ ] add date/time picker for timestamp field
/// - [ ] build account feature & add account_id as foreign key
/// - [ ] build envelope feature & add spent_from as nullable foreign key
#[component]
pub fn New(action: MultiAction<TransactionNew, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <MultiActionForm action>
            <Input name="payee".to_string() label="Payee:".to_string() attr:required=true />
            <Input name="description".to_string() label="Description:".to_string() />
            <InputAmount name="amount".to_string() label="Amount:".to_string() attr:required=true />
            <button type="submit">Create</button>
        </MultiActionForm>
    }
}

/// UI for displaying the `<li>` memebers of a list of transactions
///
/// Intended to be used to inside a `<ul>`, like so:
///
/// ```
/// <ul>
///     <ListItems transactions={...}>
/// </ul>
/// ```
///
/// Currently just displays all transactions in the database. Needs the following features
/// implemented later:
#[component]
fn ListItems(transactions: Vec<Transaction>) -> impl IntoView {
    transactions
        .into_iter()
        .map(move |transaction| {
            let Transaction {
                payee,
                amount,
                description,
                ..
            } = transaction;
            view! { <Item payee amount description /> }
        })
        .collect_view()
}

/// Component for rendering a single item in a transaction list
#[component]
fn Item(payee: String, amount: Decimal, description: Option<String>) -> impl IntoView {
    view! {
        <li>
            <ul>
                <li>{payee}</li>
                <li>{amount.to_string()}</li>
                <li>{description}</li>
            </ul>
        </li>
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

/// Server endpoint for reading all transaction
#[server(prefix = "/api", endpoint = "transactions/read/all")]
pub async fn transactions_read_many() -> Result<Vec<Transaction>, ServerFnError> {
    let pool = &pool()?;

    db_read_many(pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/// A root component for rendering a list of transactions & a form for adding new ones to the list.
/// Optimistically updates w/ pending transactions as new ones are created and before a response
/// from the server is received.
///
/// TODO:
///
/// - [ ] paginate or infinite scroll
/// - [x] auto-update if new transactions are added in the currently visible range of transactions
/// - [ ] auto-update a transaction's displayed info if it is updated in the db & it is currently
///       visible
/// - [ ] sort by columns
/// - [ ] filter by columns
#[component]
pub fn All() -> impl IntoView {
    // action for adding new transactions & signal tracking pending submission on that action
    let new = create_server_multi_action::<TransactionNew>();
    let submissions = new.submissions();

    // resource for loading all transaction saved in the db
    // updates every time the new action is executed
    let transactions = create_resource(
        move || new.version().get(),
        move |_| transactions_read_many(),
    );

    view! {
        <New action=new />
        <Suspense fallback=move || view! {<p>Loading...</p>}.into_view()>
            {move || {
                let existing_transactions = move || {
                    transactions.get().map(move |t| match t {
                        Err(err) => {
                            view! { <pre>Error fetching transactions: {err.to_string()}</pre>}.into_view()
                        },
                        Ok(transactions) => {
                            if transactions.is_empty() {
                                view! {<p>No transactions yet...</p>}.into_view()
                            } else {
                                view! { <ListItems transactions /> }.into_view()
                            }
                        }
                    }).unwrap_or_default()
                };

                // optimistically render transactions that have been submitted, but not yet
                // received back from the server
                let pending_transactions = move || {
                    submissions
                        .get()
                        .into_iter()
                        .filter(|s| s.pending().get())
                        .map(|s| s.input.get().map(|submission| {
                            let TransactionNew {payee, amount, description} = submission;
                            // convert empty strings to None, otherwise pass as Some(..)
                            let desc_option = match description.as_str() {
                                "" => None,
                                _ => Some(description),
                            };

                            view! { <Item payee amount description=desc_option /> }
                        })).collect_view()
                };

                view! {
                    <ul>
                        {pending_transactions}
                        {existing_transactions}
                    </ul>
                }
            }}
        </Suspense>
    }
}
