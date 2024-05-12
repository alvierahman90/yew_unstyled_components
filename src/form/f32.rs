use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
pub use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// HTML `name` attribute
    pub name: String,
    /// The callback to be called when the value is changed
    pub callback: Callback<f32>,
    #[prop_or_default]
    /// HTML `placeholder` attribute
    pub placeholder: Option<String>,
    #[prop_or_default]
    /// Add a `label` tag to the left of the input, is not created if `None` (default: None)
    pub label: Option<String>,
    #[prop_or_default]
    /// HTML `step` attribute (default: None)
    pub step: Option<f32>,
    #[prop_or_default]
    /// HTML `value` attribute when first loaded (default: None)
    pub value: Option<f32>,
}

#[function_component]
pub fn FormF32(props: &Props) -> Html {
    let name = props.name.clone();
    let callback = props.callback.clone();
    let placeholder = props.placeholder.clone();
    let step = props.step.map(|step| step.to_string());
    let value = props.value.map(|value| value.to_string());

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
        <input type="number" {step} {name} {onchange} {placeholder} {value}/>
        </>
    }
}
