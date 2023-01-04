use leptos::*;
// use leptos_meta::*;

#[component]
pub fn Input(cx: Scope, name: String, label: String) -> impl IntoView {
    view! {
        cx,
        <label for={&name}>{&label}":"</label>
        <input id={&name} name={&name} type="text" />
    }
}
