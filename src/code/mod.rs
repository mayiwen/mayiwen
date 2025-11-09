
use crate::myw::{self};
use dioxus::prelude::*;
mod components;
mod firefox;
mod input;
mod pc;
#[component]
pub fn Index() -> Element {
    let tabs = vec![
         myw::Tab {
                        title: "自制组件".to_string(),
            content: components::Index(),
        },
        myw::Tab {
            title: "客户端".to_string(),
            content: pc::Index(),
        },
        myw::Tab {
            title: "输入法".to_string(),
            content: input::Index(),
        },
        myw::Tab {
            title: "火狐插件".to_string(),
            content: firefox::Index(),
        },
    ];

    rsx! {
        div {
            h1 {
                "代码 "
                span { style: "font-size: 18px; display: inline-block;", "自己用的软件" }

            }
            myw::Tabset { tabs }
        }
    }
}
#[derive(Routable, Clone)]
pub enum CodeRoutes {
    #[route("/")]
    Index {},
    //     #[route("/analytics")]
    //     Analytics {},
    //     #[route("/settings")]
    //     Settings {},
}
