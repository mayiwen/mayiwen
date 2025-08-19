use std::fmt::Display;

use crate::myw::icon::Myw;
use crate::myw::{self};
use dioxus::events::KeyboardEvent;
use dioxus::html::h3;
use dioxus::{html::view, prelude::*};
use keyboard_types::Key;
use serde::{self, Deserialize, Serialize};
use web_sys;

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