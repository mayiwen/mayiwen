use std::fmt::Display;

use crate::myw::{self, Tab};
use dioxus::events::KeyboardEvent;
use dioxus::html::h3;
use dioxus::{html::view, prelude::*};
use keyboard_types::Key;
use serde::{self, Deserialize, Serialize};
use web_sys;
mod button;
mod icon;
mod message;
mod modal;
mod table;
#[component]
pub fn Index() -> Element {
    let tabs = vec![
        myw::Tab {
            title: "message".to_string(),
            content: message::Index(),
        },
        myw::Tab {
            title: "modal".to_string(),
            content: modal::Index(),
        },
        myw::Tab {
            title: "table".to_string(),
            content: table::Index(),
        },
        myw::Tab {
            title: "button".to_string(),
            content: button::Index(),
        },
        myw::Tab {
            title: "icon".to_string(),
            content: icon::Index(),
        },
    ];

    rsx! {
        div {
            myw::Gap { h: "8"  }

            h2 {
                "这个网站用到的组件"
            }
            myw::Gap {h:"8" }
            myw::Tabset { tabs }
        }
    }
}
