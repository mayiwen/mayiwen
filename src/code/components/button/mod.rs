
use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    rsx! {
        myw::Gap{},
        div {
           myw::Button { "默认按钮" }, myw::Gap { w: "8" },
           myw::Button { active: true, "按钮选中" }, myw::Gap { w: "8" },
           myw::Button { border: "none", "无边框按钮" }
        }
    }
}