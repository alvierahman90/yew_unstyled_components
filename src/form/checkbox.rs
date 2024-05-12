use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// HTML `name` attribute
    pub name: String,
    /// The callback to be called when the value is changed
    pub callback: Callback<bool>,
    #[prop_or_default]
    /// Add a `label` tag to the left of the input, is not created if `None` (default: None)
    pub label: Option<String>,
    /// HTML `checked` attribute on first load (default: false)
    #[prop_or_default]
    pub checked: bool,
}

#[function_component]
pub fn FormCheckbox(props: &Props) -> Html {
    let callback = props.callback.clone();
    let name = props.name.clone();
    let checked = props.checked;

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
        <input type="checkbox" {name} {onchange} {checked}/>
        </>
    }
}
