use chrono::naive::NaiveDateTime;
use leptos::*;

use crate::components::input::{Input, InputType};

/// An input component for entering a date & time
#[component]
pub fn DateTimePicker(
    name: String,
    label: String,
    value: NaiveDateTime,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let value = value.format("%Y-%m-%dT%H:%M:%S").to_string();

    view! {
        <Input {..attrs} input_type=InputType::DateTime name label value attr:step=1 />
    }
}
