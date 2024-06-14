#![allow(non_snake_case)]
mod data;
use data::{get_data, MinerStats, Stats};
use dioxus::prelude::*;

use gloo::timers::future::TimeoutFuture;
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
    let mut refresh_counter = use_signal(|| 60 as u8);
    let mut refresh_counter_toggle = use_signal(|| true);
    let mut data = use_resource(move || async move { get_data(address()).await });

    /* Auto update data in background every 1000msecs */
    use_future(move || async move {
        loop {
            TimeoutFuture::new(1000).await;
            if refresh_counter_toggle() {
                refresh_counter -= 1;
                if refresh_counter() == 0 {
                    data.restart();
                    refresh_counter.set(60);
                }
            }
        }
    });

    match &*data.read_unchecked() {
        Some(Ok(stats)) => rsx! {

            div {class: "container text-begin",
                div {class:"row align-items-start",
                    div {class:"col",
                        div {class:"card text-bg-light m-1 mt-2", style:"min-width: 30rem; min-height: 3rem;",
                            div {class:"card-title m-2",
                                div {class:"row",
                                    div{class:"col", b {"{address.clone()}"}},
                                    div{class:"col-auto",
                                        div{class:"row",
                                            div {class:"col",
                                                div {class:"row",
                                                    div {class:"col-auto", label{class:"form-check-label", "for":"flexSwitchCheckChecked", "Update in: {refresh_counter}"}}
                                                    div {class:"col-auto",
                                                        div {class:"form-check form-switch",
                                                            input {class:"form-check-input", "type":"checkbox", role:"switch", id:"flexSwitchCheckChecked",
                                                                onclick: move |_| {
                                                                    if refresh_counter_toggle() {
                                                                        refresh_counter_toggle.set(false);
                                                                        refresh_counter.set(60);
                                                                    } else
                                                                    {
                                                                        refresh_counter_toggle.set(true);
                                                                        data.restart();
                                                                    }
                                                                } , checked:"{refresh_counter_toggle}"
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div {class:"row align-items-start",
                        div {class: "col",
                            div {class:"card text-bg-light m-1", style:"min-width: 12rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"HASHRATE"}}
                                    div {class:"row",
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "{stats.network.hashrate} Th/s"}, p {class:"card-text", "Network"}}
                                        }
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "{stats.pool.hashrate} Gh/s"}, p {class:"card-text", "Pool"}}
                                        }
                                    }
                                }
                        },
                        div {class: "col",
                            div {class:"card text-bg-light m-1", style:"min-width: 12rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"MINERS"}}
                                    div {class:"card-body", h4 {class:"card-text m-2", "{stats.pool.connected_miners}"}}
                                }
                        },
                        div {class: "col",
                            div {class:"card text-bg-light m-1", style:"min-width: 12rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"BLOCK"}}
                                    div {class:"row",
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "{stats.network.reward} Σ"}, p {class:"card-text", "Reward"}}
                                        }
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "${stats.network.price}"}, p {class:"card-text", "ERG / SigUSD"}}
                                        }
                                    }
                            }
                        },
                        div {class: "col",
                            div {class:"card text-bg-light m-1", style:"min-width: 12rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"CURRENT"}}
                                    div {class:"row",
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "{stats.pool.effort}%"}, p {class:"card-text", "Pool Effort"}}
                                        }
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "{stats.miner.round_contribution}%"}, p {class:"card-text", "Participation"}}
                                        }
                                    }
                                }
                        },
                    },

                div {class:"row align-items-start",
                    div {class:"col",
                        div {class:"card text-bg-light m-1", style:"min-width: 30rem; min-height: 8rem;",
                            div {class:"card-title m-2", b {"MINER STATS"}}
                            div {class:"row justify-content-center",
                                div {class: "col",
                                        div {class:"card-body",
                                        h4 {class:"card-text m-2", "{stats.miner.hashrate_current} Mh/s"}
                                            p {class:"card-text m-2", "Current"}
                                            }
                                    }
                                div {class: "col",
                                        div {class:"card-body",
                                            h4 {class:"card-text m-2", "{stats.miner.hashrate_6h} Mh/s"}
                                            p {class:"card-text m-2", "6h Average"}
                                            }
                                    }
                                div {class: "col",
                                        div {class:"card-body",
                                            h4 {class:"card-text m-2", "{stats.miner.hashrate_24h} Mh/s"}
                                            p {class:"card-text m-2", "24h Average"}
                                            }
                                }
                                div {class: "col",
                                div {class:"card-body",
                                    h4 {class:"card-text m-2", "{stats.miner.paid_24h} Σ"}
                                    p {class:"card-text m-2", "24h Paid"}
                                    }
                        }
                                }
                        }
                    }
                }
                {Chart()}
                {WorkerTable(stats.clone())}
            }
        },
        Some(Err(error)) => rsx! { h1 { "Loading failed! Error: {error}"}},
        None => {
            rsx! { div {class:"d-flex justify-content-center", div {class:"spinner-border", role:"status", span{class:"visually-hidden", "Loading..."}}}}
        }
    }
}

fn Chart() -> Element {
    rsx!(
        div {class:"row justify-content-center",
        div {class:"col",
                        div {class:"card text-bg-light m-1", style:"min-width: 30rem; min-height: 24rem;",
                            div {class:"card-title m-2", b {"Chart"}
                        }
                    }
                }
            }
    )
}

fn WorkerTable(stats: Stats) -> Element {
    let mut active_workers: u8 = 1;
    let mut total_hashrate: f64 = 0.0;
    rsx!(
        div { class:"mt-3 ms-1",
            h3 {" Active workers: {stats.miner.workers.len()}"}
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
                            th{ scope: "row", "{active_workers}"}
                            td{"{worker.name}"}
                            td{"{worker.hashrate} Mh/s"}
                        }
                        {active_workers += 1;
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
    )
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

    footer {class: "footer mt-auto py-3 bg-body-tertiary ",

    div {class:"container text-center",
            div{class:"row",
                div {class:"col",
                    span {class:"text-body-secondary", "© 2021-2024 Sigmanauts Mining Pool"}
                }
                div {class:"col",
                    span {class:"text-body-secondary", "Build with ❤️ from the Ergo Community"}
                }
                div {class:"col",

                    a {class:"icon m-2", href:"https://github.com/Nederg/smp-dx",
                        svg {
                            xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"black", class:"bi bi-github", "viewBox":"0 0 16 16",
                            path {d:"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8"}
                        }
                    }

                    a {class:"icon m-2", href:"https://discord.com/channels/668903786361651200/1153460448214122526",
                        svg {
                            xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"black", class:"bi bi-discord", "viewBox":"0 0 16 16",
                            path {d:"M13.545 2.907a13.2 13.2 0 0 0-3.257-1.011.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.2 12.2 0 0 0-3.658 0 8 8 0 0 0-.412-.833.05.05 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.04.04 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032q.003.022.021.037a13.3 13.3 0 0 0 3.995 2.02.05.05 0 0 0 .056-.019q.463-.63.818-1.329a.05.05 0 0 0-.01-.059l-.018-.011a9 9 0 0 1-1.248-.595.05.05 0 0 1-.02-.066l.015-.019q.127-.095.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.05.05 0 0 1 .053.007q.121.1.248.195a.05.05 0 0 1-.004.085 8 8 0 0 1-1.249.594.05.05 0 0 0-.03.03.05.05 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019 13.2 13.2 0 0 0 4.001-2.02.05.05 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.03.03 0 0 0-.02-.019m-8.198 7.307c-.789 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.45.73 1.438 1.613 0 .888-.637 1.612-1.438 1.612m5.316 0c-.788 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.451.73 1.438 1.613 0 .888-.631 1.612-1.438 1.612"}
                        }
                    }

                    a {class:"icon m-2", href:"https://github.com/Nederg/smp-dx",
                        svg {
                            xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"black", class:"bi bi-telegram", "viewBox":"0 0 16 16",
                            path {d:"M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0M8.287 5.906q-1.168.486-4.666 2.01-.567.225-.595.442c-.03.243.275.339.69.47l.175.055c.408.133.958.288 1.243.294q.39.01.868-.32 3.269-2.206 3.374-2.23c.05-.012.12-.026.166.016s.042.12.037.141c-.03.129-1.227 1.241-1.846 1.817-.193.18-.33.307-.358.336a8 8 0 0 1-.188.186c-.38.366-.664.64.015 1.088.327.216.589.393.85.571.284.194.568.387.936.629q.14.092.27.187c.331.236.63.448.997.414.214-.02.435-.22.547-.82.265-1.417.786-4.486.906-5.751a1.4 1.4 0 0 0-.013-.315.34.34 0 0 0-.114-.217.53.53 0 0 0-.31-.093c-.3.005-.763.166-2.984 1.09"}
                        }
                    }
                }



            }


            }
        }

    }
}
