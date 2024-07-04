use crate::data::*;
use dioxus::prelude::*;
use gloo::timers::future::TimeoutFuture;

#[component]
pub fn Blocks() -> Element {
    let mut refresh_counter = use_signal(|| 5 as u8);
    let mut data = use_resource(move || async move { get_block_data().await });

    /* Auto update data in background every 1000msecs */
    use_future(move || async move {
        loop {
            TimeoutFuture::new(1000).await;
            refresh_counter -= 1;
            if refresh_counter() == 0 {
                data.restart();
                refresh_counter.set(5);
            }
        }
    });

    match &*data.read_unchecked() {
        Some(Ok(block_stats)) => rsx!(
            div { class:"mt-3 ms-1 justify-content-center",
                h3 {" Pool Blocks"}
                table {class: "table table-hover ",

                        thead {
                            tr{
                                th{ scope: "col", "Created"}
                                th{ scope: "col", "Height"}
                                th{ scope: "col", "Effort"}
                                th{ scope: "col", "Reward"}
                                th{ scope: "col", "Status"}
                                th{ scope: "col", "Miner"}
                            }
                        }
                    tbody {
                            for block in block_stats.blocks.iter(){
                                tr{
                                    if block.created != "" {
                                        td{"{block.created}"}
                                        td{"{block.block_height}"}
                                        td{"{block.effort}%"}
                                        td{"{block.block_reward} Σ"}

                                        if block.confirmation_progress == 0.0 && block.block_reward == 0.0
                                        {
                                            td{"Orphan"}
                                        }
                                        else if block.confirmation_progress == 1.0
                                        {
                                            td{"Confirmed"}
                                        }
                                        else {
                                            td{"{block.confirmation_progress}"}
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
        None => rsx!("Loading..."),
    }
}
