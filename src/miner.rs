use crate::chart::Chart;
use crate::data::*;
use crate::workertable::WorkerTable;
use dioxus::prelude::*;
use gloo::timers::future::TimeoutFuture;
#[component]
pub fn Miner(address: String) -> Element {
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
                            div {class:"card text-bg-light m-1", style:"min-width: 18rem; min-height: 9rem;",
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
                            div {class:"card text-bg-light m-1", style:"min-width: 18rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"BLOCK"}}
                                    div {class:"row",
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "{stats.network.reward} Σ"}, p {class:"card-text", "Reward"}}
                                        }
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "${stats.network.price}"}, p {class:"card-text", "SigUSD"}}
                                        }
                                    }
                            }
                        },
                        div {class: "col",
                            div {class:"card text-bg-light m-1", style:"min-width: 18rem; min-height: 9rem;",
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
                        div {class: "col",
                            div {class:"card text-bg-light m-1", style:"min-width: 18rem; min-height: 9rem;",
                                div {class: "card-title m-2", b {"MINER INFO"}}
                                div {class:"row",
                                    div {class:"col",
                                        div {class:"card-body", h5 {class:"card-text", "{stats.miner.paid_24h} Σ"}, p {class:"card-text", "24h Paid"}}
                                    }
                                    div {class:"col",
                                        div {class:"card-body", h5 {class:"card-text", "{stats.miner.total_paid} Σ"}, p {class:"card-text", "Total Paid"}}
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
                                        h4 {class:"card-text m-2", "{stats.miner.hashrate_12h} Mh/s"}
                                        p {class:"card-text m-2", "12h Average"}
                                    }
                                }
                                div {class: "col",
                                    div {class:"card-body",
                                        h4 {class:"card-text m-2", "{stats.miner.hashrate_24h} Mh/s"}
                                        p {class:"card-text m-2", "24h Average"}
                                    }
                                }

                            }
                        }
                    }
                }
                {Chart()}
                {WorkerTable(stats.clone())}
                br{}
                br{}
                br{}
                br{}
            }
        },
        Some(Err(error)) => rsx! { h1 { "Loading failed! Error: {error}"}},
        None => {
            rsx! {

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
                                                                    } , checked:"{false}"
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
                                div {class:"card text-bg-light m-1", style:"min-width: 18rem; min-height: 9rem;",
                                        div {class: "card-title m-2", b {"HASHRATE"}}
                                        div {class:"row placeholder-wave m-2",
                                            div {class:"col ",
                                                span {class:"placeholder col-10"},
                                                span {class:"placeholder col-6"}
                                            }
                                            div {class:"col",
                                                span {class:"placeholder col-10"},
                                                span {class:"placeholder col-6"}
                                            }
                                        }
                                    }
                            },
                            div {class: "col",
                                div {class:"card text-bg-light m-1", style:"min-width: 18rem; min-height: 9rem;",
                                        div {class: "card-title m-2", b {"BLOCK"}}
                                        div {class:"row placeholder-wave m-2",
                                            div {class:"col ",
                                                span {class:"placeholder col-4"},
                                                span {class:"placeholder col-10"}
                                            }
                                            div {class:"col",
                                                span {class:"placeholder col-6"},
                                                span {class:"placeholder col-10"}
                                            }
                                        }
                                }
                            },
                            div {class: "col",
                                div {class:"card text-bg-light m-1", style:"min-width: 18rem; min-height: 9rem;",
                                        div {class: "card-title m-2", b {"CURRENT"}}
                                        div {class:"row placeholder-wave m-2",
                                            div {class:"col ",
                                                span {class:"placeholder col-8"},
                                                span {class:"placeholder col-10"}
                                            }
                                            div {class:"col",
                                                span {class:"placeholder col-8"},
                                                span {class:"placeholder col-10"}
                                            }
                                        }
                                }
                            },
                            div {class: "col",
                                div {class:"card text-bg-light m-1", style:"min-width: 18rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"MINER INFO"}}
                                    div {class:"row placeholder-wave m-2",style:" placeholder-opacity-min: 0.2; placeholder-opacity-max: 0.5;",
                                    div {class:"col ",
                                        span {class:"placeholder col-4"},
                                        span {class:"placeholder col-10"}
                                    }
                                    div {class:"col",
                                        span {class:"placeholder col-12"},
                                        span {class:"placeholder col-10"}
                                    }
                                }
                                }
                            },
                        },


                    div {class:"row align-items-start",
                        div {class:"col",
                            div {class:"card text-bg-light m-1", style:"min-width: 30rem; min-height: 8rem;",
                                div {class:"card-title m-2", b {"MINER STATS"}}
                                div {class:"row justify-content-center placeholder-wave m-2",
                                    div {class: "col",
                                        div {class:"card-body",
                                            span {class:"placeholder w-100"}
                                            span {class:"placeholder col-4"}
                                        }
                                    }
                                    div {class: "col",
                                        div {class:"card-body",
                                            span {class:"placeholder w-100"}
                                            span {class:"placeholder col-4"}
                                        }
                                    }
                                    div {class: "col",
                                        div {class:"card-body",
                                            span {class:"placeholder w-100"}
                                            span {class:"placeholder col-4"}
                                        }
                                    }
                                    div {class: "col",
                                        div {class:"card-body",
                                            span {class:"placeholder w-100"}
                                            span {class:"placeholder col-4"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {Chart()}
                    {WorkerTable(Stats::default())}
                    br{}
                    br{}
                    br{}
                    br{}
                }
            }
        }
    }
}
