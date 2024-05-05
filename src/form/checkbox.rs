use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub callback: Callback<bool>,
    #[prop_or_default]
    pub label: Option<String>,
}

#[function_component]
pub fn FormCheckbox(props: &Props) -> Html {
    let callback = props.callback.clone();
    let name = props.name.clone();

    let onchange = Callback::from(move |ev: Event| {
        let target = ev.target().unwrap();
        let el = target.unchecked_into::<HtmlInputElement>();

        callback.emit(el.checked());
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
        <input type="checkbox" {name} {onchange} />
        </>
    }
}
