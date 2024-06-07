#![allow(non_snake_case)]
mod data;
use data::{get_data, Stats};
use dioxus::prelude::*;

use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/wallet/:address")]
    Wallet { address: String },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        div {   display:"flex", flex_direction:"column", width: "80%",
                div{    width: "50%",Router::<Route> {}
                    }
            }

    }
}

#[component]
fn Wallet(address: String) -> Element {
    rsx! {
        // Link { to: Route::Home {}, "Go to counter" }
        "Blog post {address}"
    }
}

#[component]
fn Home() -> Element {
    let mut address = use_signal(|| "".to_string());
    let mut btn = use_signal(|| false);
    let mut data = use_resource(move || async move { get_data().await });

    match &*data.read_unchecked() {
        Some(Ok(var)) => {
            rsx! {
                h1 {"API Data: {var.network.height}"}

                button {onclick: move |_| data.restart() , "CLICK FOR REFRESH"}

                div {
                    h1 { "Input: {address}" }
                    input {oninput: move |input| address.set(input.value())}
                }
            }
        }
        Some(Err(err)) => {
            rsx! { h1 {"{err}"}}
        }
        None => {
            rsx! { h1 { "Loading API Data" }}
        }
    }
}
