use leptos::*;

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
    Date,
    DateTime,
}

impl Into<String> for InputType {
    fn into(self) -> String {
        match self {
            InputType::Hidden => String::from("hidden"),
            InputType::Number => String::from("number"),
            InputType::Password => String::from("password"),
            InputType::Text => String::from("text"),
            InputType::Date => String::from("date"),
            InputType::DateTime => String::from("datetime-local"),
        }
    }
}
