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
    rsx! {
        main{class:"flex-shrink-0",


                    NavBar {},

                    div {class: "container-sm",

                            Router::<Route> {}
                        }

                    Footer{},

        }

    }
}

#[component]
fn Home() -> Element {
    let mut address = use_signal(|| "".to_string());
    let mut data = use_resource(move || async move { get_data().await });

    match &*data.read_unchecked() {
        Some(Ok(var)) => {
            rsx! {
                div {class:"alert alert-primary", role: "alert",

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
            rsx! { h1 { "Loading Data..." }}
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
fn NavBar() -> Element {
    rsx! {
        nav { class: "navbar navbar-expand-lg bg-body-tertiary rounded",

            div { class: "container-fluid",


                div {class: "collapse navbar-collapse d-lg-flex",

                        a {class: "navbar-brand col-lg-3 me-0", href: "/", "Sigmanauts Mining Pool"}

                        ul {class: "navbar-nav col-lg-6 justify-content-lg-center",
                            li {class: "nav-item", a{ class: "nav-link", href: "/", "Blocks"}}
                            li {class: "nav-item", a{ class: "nav-link", href: "/", "Donations"}}
                            li {class: "nav-item", a{ class: "nav-link", href: "/", "FAQ"}}
                            li {class: "nav-item", a{ class: "nav-link", href: "https://discord.com/channels/668903786361651200/1153460448214122526", "Support"}}

                            }
                    }

            }}
    }
}

#[component]
fn Footer() -> Element {
    rsx! {

    footer {class: "footer mt-auto py-3 bg-body-tertiary margin-top-5",

        div {class:"container justify-content-lg-center",
            span {class:"text-body-secondary", "Build with love from the Ergo Community"}
            }
        }

    }
}
