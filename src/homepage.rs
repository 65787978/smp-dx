#![allow(non_snake_case)]

use crate::data::get_landing_page_data;
use dioxus::prelude::*;
use gloo::timers::future::TimeoutFuture;

#[component]
pub fn Home() -> Element {
    let mut data = use_resource(move || async move { get_landing_page_data("".to_string()).await });
    let mut refresh_counter = use_signal(|| 60);
    let refresh_counter_toggle = use_signal(|| true);

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
        Some(Ok(stats)) => {
            rsx! {
                div {class:"row justify-content-center mt-3",
                    div {class:"col-auto",
                        img {max_width:"100px", max_height:"100px",src: "sig-logo.png"}
                    }
                }
                div {class:"row justify-content-center",
                    div {class:"col-auto",
                        div {class:"page-heading",
                            h1 { b {"Sigmanauts"}}
                        }
                    }
                }
                div {class:"row justify-content-center m-3",
                    div {class:"col-auto",
                        div {class:"page-heading text-center",
                            h3 { "A community to empower users of the Ergo blockchain"}
                        }
                    }
                }

                div {class:"row text-center",
                    div {class:"col-auto",
                        div {class:"card-body align-items-center", h5 {class:"card-text", "Welcome to the Sigmanauts pool, a DAO-driven, community-run mining pool dedicated to supporting the Ergo ecosystem. Joining us not only contributes to the Ergo community (fees go to Sigmanauts treasury) but also offers hourly bonus token payments."}}
                    }
                }
                div {class:"row justify-content-center",
                    div {class:"col",
                        div {class:"card text-bg-light mt-4", style:"min-height: 10rem; min-width: 20rem;",
                            div {class: "card-title m-2", b {"HOW TO CONNECT"}}
                            div {class:"card-body text-center",
                                div {class:"card-text", "Under 10 Gh/s:"},
                                div {class:"card-text", "URL: " u {"pool.ergo-sig-mining.net:3053"}}
                                br{}
                                div {class:"card-text", "Over 10 Gh/s:"},
                                div {class:"card-text","URL: " u{ "pool.ergo-sig-mining.net:3055"}}
                            }
                        }
                    }
                    div {class:"col",
                        div {class:" card text-bg-light mt-4", style:"min-height: 10rem;  min-width: 20rem;",
                            div {class:" card-title m-2", b {"STATS"},
                                div {class:"card-body text-center",
                                    div {class:"row",
                                        div {class:"col",
                                            p {"Network hashrate: {stats.network.hashrate} Th/s"}
                                            p {"Network difficulty: {stats.network.difficulty} P"}
                                            p {"Network height: {stats.network.height}"}
                                        }
                                        div {class:"col",
                                            p {"Pool hashrate: {stats.pool.hashrate} Gh/s"}
                                            p {"Pool blocks mined: {stats.pool.total_blocks}"}
                                            p {"Pool miners: {stats.pool.connected_miners}"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                br{}
                br{}
                br{}
                br{}
                br{}

            }
        }
        Some(Err(err)) => {
            rsx! { h1 {"{err}"}}
        }
        None => {
            rsx! {
                div {class:"row justify-content-center mt-3",
                    div {class:"col-auto",
                        img {max_width:"100px", max_height:"100px",src: "sig-logo.png"}
                    }
                }
                div {class:"row justify-content-center",
                    div {class:"col-auto",
                        div {class:"page-heading",
                            h1 { b {"Sigmanauts"}}
                        }
                    }
                }
                div {class:"row justify-content-center m-3",
                    div {class:"col-auto",
                        div {class:"page-heading text-center",
                            h3 { "A community to empower users of the Ergo blockchain"}
                        }
                    }
                }

                div {class:"row text-center",
                    div {class:"col-auto",
                        div {class:"card-body align-items-center", h5 {class:"card-text", "Welcome to the Sigmanauts pool, a DAO-driven, community-run mining pool dedicated to supporting the Ergo ecosystem. Joining us not only contributes to the Ergo community (fees go to Sigmanauts treasury) but also offers hourly bonus token payments."}}
                    }
                }
                div {class:"row justify-content-center",
                    div {class:"col",
                        div {class:"card text-bg-light mt-4", style:"min-height: 13rem; min-width: 20rem;",
                            div {class: "card-title m-2", b {"HOW TO CONNECT"}}
                            div {class:"card-body text-center",
                                div {class:"card-text placeholder-wave",
                                    span {class:"placeholder w-75"}
                                    span {class:"placeholder w-75"}
                                },
                                br{}
                                div {class:"card-text placeholder-wave",
                                    span {class:"placeholder w-75"}
                                    span {class:"placeholder w-75"}
                                },
                            }
                        }
                    }
                    div {class:"col",
                        div {class:" card text-bg-light mt-4", style:"min-height: 13rem;  min-width: 20rem;",
                            div {class:" card-title m-2", b {"STATS"},
                                div {class:"card-body text-center",
                                    div {class:"row",
                                        div {class:"col",
                                            p {class:"card-text placeholder-wave",
                                                span {class:"placeholder w-100"}
                                                span {class:"placeholder w-100"}
                                                span {class:"placeholder w-100"}
                                            }

                                        }
                                        div {class:"col",
                                            p {class:"card-text placeholder-wave",
                                                span {class:"placeholder w-100"}
                                                span {class:"placeholder w-100"}
                                                span {class:"placeholder w-100"}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                br{}
                br{}
                br{}
                br{}
                br{}

            }
        }
    }
}
