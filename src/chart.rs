use dioxus::prelude::*;

pub fn Chart() -> Element {
    rsx!(
        div {class:"row justify-content-center",
        div {class:"col",
                        div {class:"card text-bg-light m-1", style:"min-width: 30rem; min-height: 24rem;",
                            div {class:"card-title m-2", b {"Chart"}
                        }
                    }
                }
            }
    )
}
