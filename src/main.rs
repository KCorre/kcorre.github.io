#![recursion_limit = "1024"]
#[macro_use]
extern crate lazy_static;

use yew::prelude::*;

mod routes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <routes::BaseRouter/>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
