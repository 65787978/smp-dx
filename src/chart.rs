use crate::data::MinerStats;
use dioxus::prelude::*;

pub fn Chart(miner_data: MinerStats) -> Element {
    // let mut x_axis = vec![];
    // let mut y_axis = vec![];

    let mut x_axis = use_signal(|| vec![]);
    let mut y_axis = use_signal(|| vec![]);

    for data in miner_data.hashrate_collection.iter() {
        let hour = data.0;
        let hashrate = data.1;

        x_axis.push(format!("{hour}:00 "));
        y_axis.push(format!("{hashrate} "));
    }

    // Create a future that will resolve once the javascript has been successfully executed.
    let future = use_resource(move || async move {
        // The `eval` is available in the prelude - and simply takes a block of JS.
        // Dioxus' eval is interesting since it allows sending messages to and from the JS code using the `await dioxus.recv()`
        // builtin function. This allows you to create a two-way communication channel between Rust and JS.
        let mut chart = eval(
            r#"

                    let x_axis_data = await dioxus.recv();
                    let y_axis_data = await dioxus.recv();


                    var ctx = document.getElementById('myChart');

                    new Chart(ctx, {
                        type: 'line',
                        data: {
                            labels: x_axis_data,
                            datasets: [{
                                label: 'Miner Hashrate',
                                data:  y_axis_data,
                                borderColor: 'rgba(238, 238, 238, 0.93)',
                                tension: 0.5,
                                borderWidth: 2,
                                pointStyle: false,
                                fill: true
                            }]
                        },
                        options: {
                            scales: {
                                y: {
                                    beginAtZero: true,
                                    max: Math.round(y_axis_data[0] * 1.5)
                                }
                            }
                        }
                    });
                "#,
        );

        // Send a message to the JS code.
        chart.send(x_axis().into()).unwrap();
        chart.send(y_axis().into()).unwrap();

        // Our line on the JS side will log the message and then return "hello world".
        let res = chart.recv().await.unwrap();

        res
    });

    rsx!(
                div {class:"row justify-content-center",
                    div {class:"col m-3",
                        div {class:"card text-bg m-1",
                            div {class:"card-title m-2", b {"MINER HASHRATE"}}
                            div {style:"min-width: 20rem; min-height: 20rem;",
                                canvas {id: "myChart"}

                                {    match future.value().as_ref() {
                                        Some(chart) => rsx!("{chart}"
                                                        ),
                                        _ => rsx!( p {  } ),
                                    }
                                }
                            }
                        }
                    }
                }
    )
}
