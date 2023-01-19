// use std::{error::Error, rc::Rc};

use leptos::*;
use leptos_meta::*;
use leptos_router::{
    // Form,
    // FormProps
    Route,
    RouteProps,
    Router,
    RouterProps,
    Routes,
    RoutesProps,
};

use crate::components::{form::*, input::*};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);
    // Count button handler
    // let on_click = move |_: MouseEvent| set_count.update(|count| *count += 1);
    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/style.css"/>

        // sets the document title
        <Title text="Welcome to Hoops"/>

        // content for this welcome page
        <main>
            <Router>
                <Routes>
                <Route path="" view=|cx| view! {
                    cx,
                    <Test />
                }/>
                </Routes>
            </Router>
        </main>
    }
}

#[component]
fn Test(cx: Scope) -> impl IntoView {
    // let error_signal = create_rw_signal::<Option<Box<dyn Error>>>(cx, None);
    // let handle_login = Rc::new(|_: &'_ web_sys::Response| {});

    // // TODO: unsure what to do here, it feels like the `error` prop on `Form` is unusable due to
    // // this. Might just have to handle it all on in the `handle_login` handler instead and read
    // // error states from the `Response` object.
    // let error_alert = match error_signal.get() {
    //     None => view! { cx, <div class="hidden"></div> },
    //     Some(err) => view! {
    //         cx,
    //         <div>
    //             <h3>"Error!"</h3>
    //             <p>{format!("{:#?}", err)}</p>
    //         </div>
    //     },
    // };

    view! {
        cx,
        <Form action="/">
            <Input name="username".to_string() label="Username".to_string() />
            <Input
                name="password".to_string()
                label="Password".to_string()
                input_type=InputType::Password
            />
            <button type="submit">"Login"</button>
        </Form>
        // {error_alert}
        // <Form action="/api/token" method="POST" on_response=handle_login error=error_signal>
        //     <Input name="username".to_string() label="Username".to_string() />
        //     <Input
        //         name="password".to_string()
        //         label="Password".to_string()
        //         input_type=InputType::Password
        //     />
        //     <button type="submit">"Login"</button>
        // </Form>
        // {error_alert}
    }
}
