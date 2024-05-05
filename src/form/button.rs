use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub callback: Callback<()>,
}

#[function_component]
pub fn FormButton(props: &Props) -> Html {
    let callback = props.callback.clone();

    let onclick = Callback::from(move |_| {
        callback.emit(());
    });
    html! {
      <button {onclick}>{&props.text}</button>
    }
}
