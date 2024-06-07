use yew::{function_component, html, Html, Properties, props, Callback, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("Bob"))]
    pub name: AttrValue,
}

#[function_component(HelloWorld)]
fn hello_world(props: &Props) -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! {
        <div>
          <p>{"Hello world "}{props.name.clone()}</p>
          <button {onclick}>{ "Click" }</button>
        </div>}
}


#[function_component(App)]
pub fn app() -> Html {
    

    html! {<HelloWorld />}
}
