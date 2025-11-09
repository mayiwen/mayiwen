use dioxus::prelude::*;
pub mod core;
use crate::myw;
#[component]
pub fn Index() -> Element {
    let navigator = use_navigator();
    rsx! {
        div {
            style: "text-align: center",
            myw::Gap {  }
            core::Logo {  }
            myw::Gap {  }
            core::Search {  }
           myw::Gap { h:"12" }
            p {
                a {

                    style:  "text-decoration: underline;  cursor: pointer;",
                     onclick: move |_| {
                        navigator.push(crate::Route::Nuoruo {  });
                    },
                    title: "一文小说阅读器，待发布，前击前往了解一下吧。",
                    "前往一文小说阅读器"
                }
            }

            myw::Gap { h:"12" }
            core::Nav {  }
        }

    }
}
