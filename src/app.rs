use yew::{function_component, html, use_state, virtual_dom::AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("Bob"))]
    pub name: AttrValue,
}

#[function_component(HelloWorld)]
fn hello_world(props: &Props) -> Html {
    let message = use_state(|| AttrValue::from("Hello "));

    let onclick = {
        let message = message.clone();
        Callback::from(move |_| {
            let greeting = String::from("Hi there");
            message.set(AttrValue::from("Ola "));
            web_sys::console::log_1(&greeting.into());
        })
    };

    html! {
        <div>
          <p>{(*message).clone()}{props.name.clone()}</p>
          <button {onclick}>{ "Click" }</button>
        </div>
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {<HelloWorld name={"Jake"}/>}
}
