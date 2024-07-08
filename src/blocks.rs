use crate::data::*;
use dioxus::prelude::*;
use gloo::timers::future::TimeoutFuture;

#[component]
pub fn Blocks() -> Element {
    let mut refresh_counter = use_signal(|| 60 as u8);
    let mut data = use_resource(move || async move { get_block_data().await });

    /* Auto update data in background every 1000msecs */
    use_future(move || async move {
        loop {
            TimeoutFuture::new(1000).await;
            refresh_counter -= 1;
            if refresh_counter() == 0 {
                data.restart();
                refresh_counter.set(60);
            }
        }
    });

    rsx!(
        div {class:"row align-items-center mt-3",
            div {class:"col",
                div {class:"card text-bg m-1",
                    div {class:"card-title m-2", h5 {"POOL BLOCKS"}}
                    div {class:"card-body", style:"min-width: 20rem; min-height: 20rem;",
                    match &*data.read_unchecked() {
                        Some(Ok(block_stats)) => rsx!(

                            div {style:"overflow-x:auto;",
                                table {class: "table table-hover",
                                        thead {
                                            tr{
                                                th{ scope: "col", "CREATED"}
                                                th{ scope: "col", "HEIGHT"}
                                                th{ scope: "col", "EFFORT"}
                                                th{ scope: "col", "REWARD"}
                                                th{ scope: "col", "STATUS"}
                                                th{ scope: "col", "MINER"}
                                            }
                                        }
                                    tbody {
                                            for block in block_stats.blocks.iter(){
                                                tr{
                                                    if block.created != "" {
                                                        td{"{block.created}"}
                                                        td{"{block.block_height}"}

                                                        if block.effort < 100.0 {
                                                            td {
                                                                div {class:"progress", "role":"progressbar", style:"height: 2rem;",
                                                                    div {class:"progress-bar bg-success", style:"width: 100%", b{"{block.effort}%"}}
                                                                }
                                                            }
                                                        }
                                                        else if block.effort > 100.0 && block.effort < 200.0 {
                                                            td {
                                                                div {class:"progress", "role":"progressbar", style:"height: 2rem;",
                                                                    div {class:"progress-bar bg-warning", style:"width: 100%", b{"{block.effort}%"}}
                                                                }
                                                            }
                                                        }
                                                        else {
                                                            td {
                                                                div {class:"progress", "role":"progressbar", style:"height: 2rem;",
                                                                    div {class:"progress-bar bg-danger", style:"width: 100%", b{"{block.effort}%"}}
                                                                }
                                                            }
                                                        }

                                                        td{"{block.block_reward} Σ"}

                                                        if block.confirmation_progress == 0.0 && block.block_reward == 0.0
                                                        {
                                                            td{
                                                                div {class:"progress", "role":"progressbar", style:"height: 2rem;",
                                                                    div {class:"progress-bar bg-danger", style:"width: 100%", b{"Confirmed"}}
                                                                }
                                                            }
                                                        }
                                                        else if block.confirmation_progress == 100.0
                                                        {
                                                            td{
                                                                div {class:"progress", "role":"progressbar", style:"height: 2rem;",
                                                                    div {class:"progress-bar bg-success", style:"width: 100%", b{"Confirmed"}}
                                                                }
                                                            }
                                                        }
                                                        else {
                                                            td {
                                                                div {class:"progress", "role":"progressbar", style:"height: 2rem;",
                                                                    div {class:"progress-bar progress-bar-striped progress-bar-animated bg-success", style:"width: {block.confirmation_progress}%", b{"{block.confirmation_progress}%"}}
                                                                }
                                                            }
                                                        }

                                                        td{"{block.miner}"}
                                                    }

                                                }
                                            }
                                        }
                                    }
                                }
                        ),
                        Some(Err(error)) => rsx! { h1 { "Loading failed! Error: {error}"}},
                        None =>
                            rsx!(
                                div { class:"mt-3 ms-1 placeholder-wave",
                                    table {class: "table table-hover placeholder-wave",

                                            thead {
                                                tr{
                                                    th{ scope: "col", "CREATED"}
                                                    th{ scope: "col", "HEIGHT"}
                                                    th{ scope: "col", "EFFORT"}
                                                    th{ scope: "col", "REWARD"}
                                                    th{ scope: "col", "STATUS"}
                                                    th{ scope: "col", "MINER"}

                                                }
                                            }

                                            tbody {
                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }

                                                tr{
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}
                                                    td{span {class:"placeholder col-8 m-2"}}

                                                }
                                            }
                                        }
                                }
                            ),
                    }
                    }
                }
            }
        }
    )
}
