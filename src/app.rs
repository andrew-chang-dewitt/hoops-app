use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
        <TransactionCreate />
    }
}

/// add Transaction server endpoint
#[server(prefix = "/api", endpoint = "transaction/create")]
pub async fn transaction_create(
    desc: String,
    payee: String,
    amount: f32,
) -> Result<(), ServerFnError> {
    println!("{}, {}, {}", desc, payee, amount);
    Ok(())
}

/// UI for adding a transaction to the record
#[component]
fn TransactionCreate() -> impl IntoView {
    let transaction_create = create_server_action::<TransactionCreate>();

    view! {
        <ActionForm action=transaction_create>
            <Input name="payee".to_string() label="Payee:".to_string() />
            <Input name="desc".to_string() label="Description:".to_string() />
            <InputAmount name="amount".to_string() label="Amount:".to_string() />
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
