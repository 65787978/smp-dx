#![allow(non_snake_case)]
mod data;
use data::get_data;
use dioxus::prelude::*;

use std::thread;
use std::time::Duration;
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
    // let mut data = use_resource(move || async move { get_data().await });

    // match &*data.read_unchecked() {
    //     Some(Ok(var)) => {
    //         rsx! {
    //             div {class:"alert alert-primary", role: "alert",

    //                 div {class: "row align-items-center",
    //                     div {class: "col", width: "80%", input {class: "form-control form-control-lg", placeholder:"Enter your mining address here...",  oninput: move |input| address.set(input.value())}}
    //                     div {class: "col-auto", Link {class: "btn btn-primary", to: Route::Wallet { address: address() }, "Search"}}

    //                 }

    //             }

    //             div {class: "alert alert-primary", role: "alert", "Network hashrate: {var.network.hashrate.back().unwrap_or(&(0.0, 0.0)).1} Th/s"}
    //             h1 {"Network hashrate: {var.network.hashrate.back().unwrap_or(&(0.0, 0.0)).1} Th/s"}
    //             h1 {"Network difficulty: {var.network.difficulty} P"}
    //             h1 {"Network height: {var.network.height} "}
    //             h1 {"Pool hashrate: {var.pool.hashrate.back().unwrap_or(&(0.0, 0.0)).1} Gh/s"}
    //             h1 {"Pool connected miners: {var.pool.connected_miners}"}
    //             h1 {"Pool effort: {var.pool.effort}%"}
    //             h1 {"Pool total blocks: {var.pool.total_blocks}"}

    //             button {class: "btn btn-primary", onclick: move |_| data.restart() , "CLICK FOR REFRESH"}
    //         }
    //     }
    //     Some(Err(err)) => {
    //         rsx! { h1 {"{err}"}}
    //     }
    //     None => {
    //         rsx! { div {class:"d-flex justify-content-center", div {class:"spinner-border", role:"status", span{class:"visually-hidden", "Loading..."}}}
    //         }
    //     }
    // }
    rsx! {
            div {class:"alert alert-primary", role: "alert",

                        div {class: "row align-items-center",
                            div {class: "col", width: "80%", input {class: "form-control form-control-lg", placeholder:"Enter your mining address here...",  oninput: move |input| address.set(input.value())}}
                            div {class: "col-auto", Link {class: "btn btn-primary", to: Route::Wallet { address: address() }, "Search"}}

                        }
        }
    }
}

#[component]
fn Wallet(address: String) -> Element {
    let address = use_signal(|| address);
    let data = use_resource(move || async move { get_data(address()).await });

    let mut num: u8 = 1;
    let mut total_hashrate: f64 = 0.0;

    match &*data.read_unchecked() {
        Some(Ok(stats)) => rsx! {
            h1 {"Address: {address.clone()}"}

            div { h1 { "Average 24h hashrate: {stats.miner.average_hashrate}"}}

            div { "Miner Hashrate: {stats.miner}"}


            div {
                table {class: "table table-hover",

                        thead {
                            tr{
                                th{ scope: "col", "#"}
                                th{ scope: "col", "Worker Name"}
                                th{ scope: "col", "Hashrate"}
                            }
                       }

                       tbody {


                            for worker in stats.miner.workers.iter(){
                            tr{
                                th{ scope: "row", "{num}"}
                                td{"{worker.name}"}
                                td{"{worker.hashrate} Mh/s"}
                            }
                            {num += 1;
                                total_hashrate += worker.hashrate}
                            }
                        }
                        thead {
                            tr{
                                th{ scope: "col", "Total:"}
                                th{ scope: "col", ""}
                                th{ scope: "col", "{(total_hashrate * 100.0).round() / 100.0} Mh/s"}
                            }
                       }
                    }
            }
        },
        Some(Err(error)) => rsx! { h1 { "Loading failed! Error: {error}"}},
        None => {
            rsx! { div {class:"d-flex justify-content-center", div {class:"spinner-border", role:"status", span{class:"visually-hidden", "Loading..."}}}}
        }
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav { class: "navbar navbar-expand-lg bg-body-tertiary rounded",

            div { class: "container-fluid",

            button {"class": "navbar-toggler","type":"button", "data-bs-toggle": "collapse", "data-bs-target":"#navbar", "aria-controls": "navbar", "aria-expanded":"false","aria-label":"Toggle navigation",
                    span{class:"navbar-toggler-icon"}
                }

                div {class: "collapse navbar-collapse d-lg-flex", id:"navbar",

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

    footer {class: "footer mt-auto py-3 bg-body-tertiary",

        div {class:"container justify-content-lg-center",
            span {class:"text-body-secondary", "Build with love from the Ergo Community"}
            }
        }

    }
}
