#![allow(non_snake_case)]
mod blocks;
mod chart;
mod data;
mod homepage;
mod miner;
mod workertable;

use blocks::Blocks;
use dioxus::prelude::*;
use homepage::Home;
use miner::Miner;
use tracing::Level;
const _: &str = manganis::mg!(file("./assets/main.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/wallet/:address")]
    Miner { address: String },
    #[route("/blocks")]
    Blocks {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        main{class:"flex-shrink-1",
                    NavBar {},

                    div {class: "container-xxl",

                            Router::<Route> {}
                        }

                    Footer{},

        }

    }
}

#[component]
fn NavBar() -> Element {
    let mut address = use_signal(|| "".to_string());
    rsx! {
        nav { class: "navbar navbar-expand-lg bg-body-tertiary rounded",
            div { class: "container-fluid",

                a {class: "navbar-brand col-lg-3 me-0", href: "/", "Sigmanauts Mining Pool"}

                button {"class": "navbar-toggler","type":"button", "data-bs-toggle": "collapse", "data-bs-target":"#navbar", "aria-controls": "navbar", "aria-expanded":"false","aria-label":"Toggle navigation",
                        span{class:"navbar-toggler-icon"}
                }

                div {class: "collapse navbar-collapse d-lg-flex", id:"navbar",

                    a {class:"nav-item col-lg-3 me-0"}

                    ul {class: "navbar-nav col-lg-6 align-items-center",
                        li {class: "nav-item", a{ class: "nav-link", href: "/blocks", "Blocks"}}
                        li {class: "nav-item", a{ class: "nav-link", href: "https://discord.com/channels/668903786361651200/1153460448214122526", "Support"}}
                        li {class: "nav-item",
                            a { class: "btn btn-outline-secondary mb-1", href: "https://explorer.ergoplatform.com/payment-request?address=9fFzKA2WHNYyXZWc4MHPtSv6YqS8jtDsZkSnAQwVaAZrYn9ojEA", "Donate",
                                svg {
                                    xmlns:"http://www.w3.org/2000/svg",width:"18", height:"18", fill:"Red", class:"bi bi-suit-heart-fill ms-2", "viewBox":"0 0 16 16",
                                    path { d:"M4 1c2.21 0 4 1.755 4 3.92C8 2.755 9.79 1 12 1s4 1.755 4 3.92c0 3.263-3.234 4.414-7.608 9.608a.513.513 0 0 1-.784 0C3.234 9.334 0 8.183 0 4.92 0 2.755 1.79 1 4 1"}}
                            }
                        }
                    }

                    form {role:"search",  action:"/wallet/{address()}",
                        div {class: "col-auto",
                            input {class: "form-control", placeholder:"Search mining address", minlength: 51, maxlength: 51, oninput: move |input| address.set(input.value())}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {

    footer {class: "footer mt-auto py-1 bg-body-tertiary fixed-bottom",
        div {class:"container text-center",
                div{class:"row",
                    div {class:"col",
                        span {class:"text-body-secondary", "© 2021-2024"}
                    }
                    div {class:"col",

                        a {class:"icon m-2", href:"https://github.com/65787978/smp-dx",
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

                        a {class:"icon m-2", href:"https://t.me/sig_mining",
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
