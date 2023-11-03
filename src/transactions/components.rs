use chrono::{DateTime, NaiveDateTime, Utc};
use leptos::{svg::tspan, *};
use leptos_router::*;
use rust_decimal::prelude::*;

use crate::components::{
    datepicker::DateTimePicker,
    input::{Input, InputAmount},
};
use crate::transactions::model::Transaction;

#[cfg(feature = "ssr")]
use crate::transactions::model::{db_insert_new, db_read_many, pool};

/// UI for adding a transaction to the record
///
/// TODO:
///
/// - [ ] error handling
/// - [x] optimistic updates to a co-located list
/// - [ ] add date/time picker for timestamp field
#[component]
pub fn New(action: MultiAction<TransactionNew, Result<(), ServerFnError>>) -> impl IntoView {
    let timestamp_value = Utc::now().naive_utc();

    view! {
        <MultiActionForm action>
            <Input name="payee".to_string() label="Payee:".to_string() attr:required=true />
            <Input name="description".to_string() label="Description:".to_string() />
            <InputAmount name="amount".to_string() label="Amount:".to_string() attr:required=true />
            // FIXME: how to pass timestamp as UTC value?
            // FIXME: try using ISO8601 formatting w/ `<input type="date">...</input>`--
            //        see
            //        https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/datetime-local
            //        for more information.
            //        This will force local time in the browser, but that can be pretty easily made
            //        into a chrono::NaiveDateTime and then UTC via that struct's `.and_utc()`
            //        method
            <DateTimePicker name="timestamp".to_string() label="Timestamp:".to_string() value=timestamp_value attr:required=true />
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
                timestamp,
                ..
            } = transaction;
            view! { <Item payee amount description timestamp /> }
        })
        .collect_view()
}

/// Component for rendering a single item in a transaction list
#[component]
fn Item(
    payee: String,
    amount: Decimal,
    description: Option<String>,
    timestamp: DateTime<Utc>,
) -> impl IntoView {
    view! {
        <li>
            <ul>
                <li>{payee}</li>
                <li>{amount.to_string()}</li>
                <li>{description}</li>
                <li>{timestamp.to_rfc2822()}</li>
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
    timestamp: String,
) -> Result<(), ServerFnError> {
    println!("timestamp is: {}", &timestamp);
    // convert empty strings to None, otherwise pass as Some(..)
    let description = match description.as_str() {
        "" => None,
        _ => Some(description),
    };
    // convert rfc_2822 datestring into DateTime
    let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%dT%H:%M")?,
        Utc,
    );
    // if getting a pool fails, immediately return the error instead of proceeding
    let pool = &pool()?;

    let transaction = Transaction::new(amount, payee, timestamp, description);
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
                            let TransactionNew {payee, amount, description, timestamp} = submission;
                            // convert empty strings to None, otherwise pass as Some(..)
                            let desc_option = match description.as_str() {
                                "" => None,
                                _ => Some(description),
                            };
                            // get DateTime from rfc_2822 datestring
                            let timestamp = DateTime::<Utc>::from(DateTime::parse_from_rfc2822(&timestamp).unwrap());

                            view! { <Item payee amount description=desc_option timestamp /> }
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
