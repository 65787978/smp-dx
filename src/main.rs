#![allow(non_snake_case)]
mod data;
use data::get_data;
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
    rsx! {  div {class: "container-sm",
                Router::<Route> {}
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
    let mut data = use_resource(move || async move { get_data().await });

    match &*data.read_unchecked() {
        Some(Ok(var)) => {
            rsx! {
                div {class:"mb-3",
                h1 { "Input: {address}" }

                    input {class: "form-control form-control-lg", placeholder:"address",

                            oninput: move |input| address.set(input.value())
                         }
                    }

                    div {class: "alert alert-primary", role: "alert", "Network hashrate: {var.network.hashrate.back().unwrap_or(&(0.0, 0.0)).1} Th/s"}
                h1 {"Network hashrate: {var.network.hashrate.back().unwrap_or(&(0.0, 0.0)).1} Th/s"}
                h1 {"Network difficulty: {var.network.difficulty} P"}
                h1 {"Network height: {var.network.height} "}
                h1 {"Pool hashrate: {var.pool.hashrate.back().unwrap_or(&(0.0, 0.0)).1} Gh/s"}
                h1 {"Pool connected miners: {var.pool.connected_miners}"}
                h1 {"Pool effort: {var.pool.effort}%"}
                h1 {"Pool total blocks: {var.pool.total_blocks}"}

                button {class: "btn btn-primary", onclick: move |_| data.restart() , "CLICK FOR REFRESH"}

                div {

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
