use crate::data::Stats;
use dioxus::prelude::*;

pub fn WorkerTable(stats: Stats) -> Element {
    let mut active_workers: u8 = 1;
    let mut total_hashrate: f64 = 0.0;

    if stats != Stats::default() {
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

                                {
                                    active_workers += 1;
                                    total_hashrate += worker.hashrate
                                }
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
    } else {
        rsx!(
            div { class:"mt-3 ms-1 placeholder-wave",
                h5 {span {class:"placeholder col-3 m-2"}}
                table {class: "table table-hover placeholder-wave",

                        thead {
                            tr{
                                th{ scope: "col", span {class:"placeholder col-5 m-2"}}
                                th{ scope: "col", span {class:"placeholder col-5 m-2"}}
                                th{ scope: "col", span {class:"placeholder col-5 m-2"}}
                            }
                       }

                       tbody {
                            tr{
                                th{ scope: "row", span {class:"placeholder col-5 m-2"}}
                                td{span {class:"placeholder col-5 m-2"}}
                                td{span {class:"placeholder col-5 m-2"}}
                            }

                        }
                        thead {
                            tr{
                                th{ scope: "col", span {class:"placeholder col-5 m-2"}}
                                th{ scope: "col", span {class:"placeholder col-5 m-2"}}
                                th{ scope: "col", span {class:"placeholder col-5 m-2"}}
                            }
                       }
                    }
            }
        )
    }
}
