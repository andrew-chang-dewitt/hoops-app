use leptos::typed_builder::*;
use leptos::{view, IntoView, Scope};

/// Properties that can be passed to the [Input] component.
#[derive(TypedBuilder)]
pub struct FormProps {}

#[allow(non_snake_case)]
pub fn Form(cx: Scope, _props: FormProps) -> impl IntoView {
    view! {
        cx,
        <form></form>
    }
}

#[cfg(test)]
mod tests {
    use leptos::mount_to_body;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // use crate::components::test_utils::*;

    use super::*;

    #[wasm_bindgen_test]
    pub fn it_renders_a_form_element_with_default_attributes() {
        mount_to_body(move |cx| {
            view! {
                cx,
                <Form>
                </Form>
            }
        });

        let document = leptos::document();
        let form = document.query_selector("form").unwrap();
        console_log!("{:#?}", form);
        println!("{:#?}", form);
        let all = document.query_selector_all("*").unwrap().length();
        console_log!("{:#?}", all);
        println!("{:#?}", all);
    }
}
