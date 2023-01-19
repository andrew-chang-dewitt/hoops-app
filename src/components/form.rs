use leptos::{component, view, Fragment, IntoView, Scope};
use leptos_router::use_resolved_path;

pub enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    pub fn to_string(&self) -> String {
        match self {
            Get => String::from("GET"),
            Post => String::from("POST"),
        }
    }
}

pub enum FormEnctype {
    XWwwFormUrlEncoded,
    Json,
}

#[component]
pub fn Form<Action>(
    cx: Scope,
    /// The url to submit the form to
    action: Action,
    /// The submission method to use, defaults to GET
    method: Option<HttpMethod>,
    /// The submission encoding type to use, defaults to application/x-www-form-urlencoded
    enctype: Option<FormEnctype>,
    /// Enable or disable autocomplete, defaults to enabled
    autocomplete: Option<bool>,
    /// Component children
    children: Box<dyn FnOnce(Scope) -> Fragment>,
) -> impl IntoView {
    let used_method = if let Some(m) = method {
        m.to_string()
    } else {
        HttpMethod::Get.to_string()
    };
    let used_action = use_resolved_path(cx, move || action);

    view! {
        cx,
        <form action=used_action method=used_method>
            {children(cx)}
        </form>
    }
}

#[cfg(test)]
mod tests {
    use leptos::mount_to_body;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // use crate::components::test_utils::*;

    use super::*;

    mod default_attributes {
        use web_sys::Element;

        use super::*;

        fn setup<Action: 'static>(action: Action) -> Option<Element> {
            mount_to_body(move |cx| {
                view! {
                    cx,
                    <Form action=action>
                        <label for="username">"Username:"</label>
                        <input id="username" type="text" />
                        <button type="submit">"Submit"</button>
                    </Form>
                }
            });

            let document = leptos::document();
            document.query_selector("form").unwrap()
        }

        #[wasm_bindgen_test]
        pub fn it_uses_the_given_action() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.get_attribute("action"),
                Some(String::from("/expected/action")),
                "Form should use the given action."
            );
        }

        #[wasm_bindgen_test]
        pub fn it_contains_the_expected_children() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.child_element_count(),
                3,
                "Form should contain only the given number of children"
            );
        }

        #[wasm_bindgen_test]
        pub fn it_defaults_to_get_method() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.get_attribute("method"),
                Some(String::from("GET")),
                "Form should default to GET method."
            );
        }

        #[wasm_bindgen_test]
        pub fn it_defaults_to_enabling_autocomplete() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.get_attribute("autocomplete"),
                Some(String::from("true")),
                "Form should default to autocomplete being enabled."
            );
        }

        #[wasm_bindgen_test]
        pub fn it_defaults_to_url_encoding() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.get_attribute("enctype"),
                Some(String::from("application/x-www-form-urlencoded")),
                "Form should use url encoding by default."
            );
        }
    }
}
