// TODO rename to FormEnumSelect or generalise into select for any iterable

use std::fmt::Display;
use std::str::FromStr;
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<T: std::cmp::PartialEq + IntoEnumIterator + Display + FromStr> {
    pub name: String,
    pub callback: Callback<Option<T>>,
    #[prop_or_default]
    pub label: Option<String>,
}

#[function_component]
pub fn FormSelect<T: std::cmp::PartialEq + IntoEnumIterator + Display + FromStr + 'static>(
    props: &Props<T>,
) -> Html {
    let callback = props.callback.clone();
    let onchange = move |e: Event| {
        let element = e.target().unwrap().unchecked_into::<HtmlSelectElement>();
        let value = T::from_str(&element.value()).ok();

        callback.emit(value);
    };
    let name = props.name.clone();

    html! {
        <>
        {
            if let Some(label) = props.label.clone() {
                html! {
                    <label for={props.name.clone()}>{label}</label>
                }
            } else {
                html!{}
            }
        }
        <select {name} {onchange}>
        {

            T::iter().map(|option| {
                let value = option.to_string();
                let display_value = value.clone();
                html! {
                    <option {value}>{display_value}</option>
                }
            }).collect::<Html>()
        }
        </select>
        </>
    }
}
