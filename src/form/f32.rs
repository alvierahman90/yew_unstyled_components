use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
pub use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub callback: Callback<f32>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub step: Option<String>,
}

#[function_component]
pub fn FormF32(props: &Props) -> Html {
    let name = props.name.clone();
    let callback = props.callback.clone();
    let placeholder = props.placeholder.clone();
    let step = props.step.clone();

    let onchange = Callback::from(move |ev: Event| {
        let target = ev.target().unwrap();
        let el = target.unchecked_into::<HtmlInputElement>();
        // TODO give message to user that input is invalid
        let value = el
            .value()
            .parse::<f32>()
            .expect("Unable to parse input value as f32");

        callback.emit(value);
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
        <input type="number" {step} {name} {onchange} {placeholder} />
        </>
    }
}
