use dioxus::prelude::*;
use dioxus_charts::LineChart;

use crate::data::MinerStats;

pub fn Chart(miner_data: MinerStats) -> Element {
    let mut x_axis = vec![];
    let mut y_axis = vec![];

    for data in miner_data.hashrate_collection.iter() {
        let hour = data.0;
        x_axis.push(format!("{hour}:00").to_string());
        y_axis.push(data.1 as f32);
    }

    rsx!(
        div {class:"row justify-content-center",
            div {class:"col",
                div {class:"card text-bg-light m-1",
                    div {class:"card-title m-2", b {"MINER HASHRATE"}}
                    div {style:"min-width: 20rem; min-height: 20rem; max-height: 25rem;",
                        LineChart {
                            padding_top: 30,
                            padding_left: 100,
                            padding_right: 80,
                            padding_bottom: 30,
                            viewbox_width: 1500,
                            viewbox_height: 450,
                            show_grid_ticks: true,
                            line_width: "0.3%",
                            dot_size: "1%",
                            label_interpolation: (|y_axis| format!("{y_axis} Mh/s")) as fn(f32) -> String,
                            series: vec![
                                y_axis,
                            ],
                            labels: x_axis,
                        }
                    }
                }
            }
        }
    )
}
