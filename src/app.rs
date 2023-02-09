use leptos::*;
use leptos_meta::*;
use leptos_router::{
    AProps, ActionForm, ActionFormProps, Route, RouteProps, Router, RouterProps, Routes,
    RoutesProps, A,
};
use leptos_server::create_server_action;

use crate::components::input::{Input, InputProps, InputType};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Hoops | App"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/>}/>
                    <Route path="/login" view=|cx| view! { cx, <Login/>}/>
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = Login::register();
}

#[server(Login, "/api")]
async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    let result = format!("Got username: {username} & password: {password}");
    println!("{result}");

    Ok(result)
}

/// Renders the login page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1>"Welcome!"</h1>
        <A href="/login">"log in"</A>
    }
}

/// Renders the login page of your application.
#[component]
fn Login(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);

    view! {
        cx,
        <ActionForm action=login>
            <Input name={ String::from( "username" ) } label={ String::from( "Username:" ) }/>
            <Input name={ String::from( "password" ) } label={ String::from( "Password:" ) } input_type=InputType::Password />
            <button type="submit">"Login"</button>
        </ActionForm>

    }
}
