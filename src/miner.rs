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

    /* Auto update data in background */
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

    let short_address = shorten_string(address().as_str(), 35);

    match &*data.read_unchecked() {
        Some(Ok(stats)) => rsx! {

                div {class:"row align-items-start",
                    div {class:"col",
                        div {class:"card text-bg m-1 mt-2", style:"min-width: 20rem; min-height: 3rem;",
                            div {class:"card-title m-2",
                                div {class:"row",
                                    div{class:"col", b {"{short_address}"}},
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
                            div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"HASHRATE"}}
                                    div {class:"row",
                                        div {class:"col",
                                            div {class:"card-body",
                                            h5 {class:"card-text", "{stats.network.hashrate} Th/s"}, p {class:"card-text", "Network"}}
                                        }
                                        div {class:"col",
                                            div {class:"card-body", h5 {class:"card-text", "{stats.pool.hashrate} Gh/s"}, p {class:"card-text", "Pool"}}
                                        }
                                    }
                                }
                        },
                        div {class: "col",
                            div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 9rem;",
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
                            div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 9rem;",
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
                        }
                    }

                div {class:"row align-items-start",
                    div {class: "col",
                        div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 8rem;",
                            div {class: "card-title m-2", b {"MINER INFO"}}

                                div {class:"row",
                                    div {class:"col m-1",
                                        div {class:"card-body", h5 {class:"card-text m-2", "{stats.miner.pending_shares}"}, p {class:"card-text m-2", "Pending Shares"}}
                                    }
                                    div {class:"col m-1",
                                        div {class:"card-body", h5 {class:"card-text m-2", "{stats.miner.paid_24h} Σ"}, p {class:"card-text m-2", "24h Paid"}}
                                    }
                                    div {class:"col m-1",
                                        div {class:"card-body", h5 {class:"card-text m-2", "{stats.miner.total_paid} Σ"}, p {class:"card-text m-2", "Total Paid"}}
                                    }
                                    div {class:"col m-1",
                                        div {class:"card-body", h5 {class:"card-text m-2", "{stats.miner.workers_number}"}, p {class:"card-text m-2", "Active Workers"}}
                                    }
                                }

                        }
                    },
                },

                div {class:"row align-items-start",
                    div {class:"col",
                        div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 8rem;",
                            div {class:"card-title m-2", b {"HASHRATE STATS"}}
                            div {class:"row justify-content-center",
                                div {class: "col m-1",
                                    div {class:"card-body",
                                        h5 {class:"card-text m-2", "{stats.miner.hashrate_current} Mh/s"}
                                        p {class:"card-text m-2", "Current"}
                                    }
                                }
                                div {class: "col m-1",
                                    div {class:"card-body",
                                        h5 {class:"card-text m-2", "{stats.miner.hashrate_6h} Mh/s"}
                                        p {class:"card-text m-2", "6h Average"}
                                    }
                                }
                                div {class: "col m-1",
                                    div {class:"card-body",
                                        h5 {class:"card-text m-2", "{stats.miner.hashrate_12h} Mh/s"}
                                        p {class:"card-text m-2", "12h Average"}
                                    }
                                }
                                div {class: "col m-1",
                                    div {class:"card-body",
                                        h5 {class:"card-text m-2", "{stats.miner.hashrate_24h} Mh/s"}
                                        p {class:"card-text m-2", "24h Average"}
                                    }
                                }

                            }
                        }
                    }
                }
                //Chart
                { Chart(stats.miner.clone()) }
                // {chart()}
                {WorkerTable(stats.clone())}
                br{}
        },
        Some(Err(error)) => rsx! { h1 { "Loading failed! Error: {error}"}},
        None => {
            rsx! {

                div {class:"row align-items-start",
                    div {class:"col",
                        div {class:"card text-bg m-1 mt-2", style:"min-width: 20rem; min-height: 3rem;",
                            div {class:"card-title m-2",
                                div {class:"row",
                                    div{class:"col", b {"{short_address}"}},
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
                            div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 9rem;",
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
                            div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"BLOCK"}}
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
                            div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 9rem;",
                                    div {class: "card-title m-2", b {"CURRENT"}}
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
                            div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 9rem;",
                                div {class: "card-title m-2", b {"MINER INFO"}}
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
                        },
                    },


                div {class:"row align-items-start",
                    div {class:"col",
                        div {class:"card text-bg m-1", style:"min-width: 20rem; min-height: 8rem;",
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
                div {class:"row align-items-start",
                div {class:"col",
                    div {class:"card text-bg m-1",
                        div {class:"card-title m-2", b {"MINER HASHRATE"}},

                        div {class:"card-body", style:"min-width: 20rem; min-height: 20rem; max-height: 25rem;",
                            h1 {span {class:"placeholder w-100"}}
                            h1 {span {class:"placeholder w-100"}}
                            h1 {span {class:"placeholder w-100"}}
                            h1 {span {class:"placeholder w-100"}}
                            h1 {span {class:"placeholder w-100"}}
                        }
                    }
                }
            }
                {WorkerTable(Stats::default())}
                br{}
                br{}
                br{}
                br{}
            }
        }
    }
}
