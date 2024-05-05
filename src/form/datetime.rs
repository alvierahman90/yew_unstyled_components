use chrono::{DateTime, NaiveDateTime, Utc};
use js_sys::Date;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub callback: Callback<DateTime<Utc>>,
    #[prop_or_default]
    pub label: Option<String>,
}

#[function_component]
pub fn FormDateTime(props: &Props) -> Html {
    let callback = props.callback.clone();
    let name = props.name.clone();
    let onchange = move |e: Event| {
        let element = e.target().unwrap().unchecked_into::<HtmlInputElement>();
        let offset_mins = Date::new_0().get_timezone_offset() as i32;
        let datetime = NaiveDateTime::parse_from_str(&element.value(), "%FT%R") // 2024-05-09T18:00
            .expect("Failed to parse time from form");
        // local user's offset from UTC
        let offset = chrono::offset::FixedOffset::east_opt(offset_mins * 60)
            .expect("Failded to create offset");
        // chrono allows you to add a timezone (NaiveDateTime::and_local_timezone)
        // to convert into a DateTime but using the DateTime::to_utc method converts the
        // time by simply replacing the timezone with Utc, rather than calculating what the time
        // in UTC would be for the time and timezone
        // Therefore we must do it ourselves with NaiveDateTime::checked_sub_offset and then
        // NaiveDateTime::and_utc
        let datetime = datetime
            .checked_sub_offset(offset)
            .expect("Failed to subtract offset")
            .and_utc();

        callback.emit(datetime);
    };

    html! {
        <>
        {
            if let Some(label) = props.label.clone() {
                html!{ <label for={props.name.clone()}>{label}</label> }
            } else {
                html!{}
            }
        }
        <input type="datetime-local" {name} {onchange} />
        </>
    }
}
