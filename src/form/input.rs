use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
pub use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub callback: Callback<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub label: Option<String>,
}

#[function_component]
pub fn FormInput(props: &Props) -> Html {
    let name = props.name.clone();
    let callback = props.callback.clone();
    let placeholder = props.placeholder.clone();

    let onchange = Callback::from(move |ev: Event| {
        let target = ev.target().unwrap();
        let el = target.unchecked_into::<HtmlInputElement>();

        callback.emit(el.value());
    });

    html! {
        <>
        {
            if let Some(label) = props.label.clone() {
                html! { <label for={props.name.clone()}>{label}</label> }
            } else {
                html!{}
            }
        }
        <input type="text" {name} {onchange} {placeholder} />
        </>
    }
}
