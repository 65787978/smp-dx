use crate::data::MinerStats;
use dioxus::prelude::*;
use dioxus_charts::LineChart;

pub fn Chart(miner_data: MinerStats) -> Element {
    let mut x_axis = vec![];
    let mut y_axis = vec![];

    for data in miner_data.hashrate_collection.iter() {
        let hour = data.0;
        let hashrate = data.1;

        x_axis.push(format!("{hour}:00 "));
        y_axis.push(hashrate as f32);
    }

    let mut highest = 0.0;
    for i in y_axis.iter() {
        if *i > highest {
            highest = *i;
        }
    }
    rsx!(
        div {class:"row align-items-start",
            div {class:"col",
                div {class:"card text-bg m-1",
                    div {class:"card-title m-2", b {"MINER HASHRATE"}}
                    div {class:"card-body", style:"min-width: 20rem; min-height: 20rem; max-height: 30rem;",

                            LineChart {
                                padding_top: 30,
                                padding_left: 100,
                                padding_right: 50,
                                padding_bottom: 30,
                                viewbox_width: 1400,
                                viewbox_height: 500,
                                lowest: 0.0,
                                highest: highest * 1.2,
                                show_grid_ticks: false,
                                line_width: "0.3%",
                                dot_size: "1%",
                                label_interpolation: (|y_axis| format!("{y_axis} Mh/s")) as fn(f32) -> String,
                                series: vec![
                                    y_axis,
                                ],
                                labels: x_axis,
                            }


                    div {
                        id: "chart",
                        style: "display: inline-block;",
                      }

                    }
                }
            }
        }
    )
}
