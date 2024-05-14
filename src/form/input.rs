use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
pub use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// HTML `name` attribute
    pub name: String,
    /// The callback to be called when the value is changed
    pub callback: Callback<String>,
    /// HTML `placeholder` attribute
    #[prop_or_default]
    pub placeholder: Option<String>,
    /// Add a `label` tag to the left of the input, is not created if `None` (default: None)
    #[prop_or_default]
    pub label: Option<String>,
    /// HTML `value` attribute (default: None)
    #[prop_or_default]
    pub value: Option<String>,
}

#[function_component]
pub fn FormInput(props: &Props) -> Html {
    let name = props.name.clone();
    let callback = props.callback.clone();
    let placeholder = props.placeholder.clone();
    let value = props.value.clone();

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
        <input type="text" {name} {onchange} {placeholder} {value}/>
        </>
    }
}
