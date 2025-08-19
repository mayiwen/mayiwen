use std::fmt::Display;

use crate::myw::{self, Tab};
use dioxus::events::KeyboardEvent;
use dioxus::html::h1;
use dioxus::{html::view, prelude::*};
use keyboard_types::Key;
use serde::{self, Deserialize, Serialize};
use web_sys;
mod about;
mod backend;
mod data;
mod link;
mod login;
mod models;
mod title;
use crate::GLOBAL_USER_AUTH;

#[component]
pub fn Index() -> Element {
    // static MAYIWEN: Asset = asset!("/assets/mayiwen/pc/mayiwen.png");
    let tabs = vec![
        // myw::Tab {
        //     title: "客户端".to_string(),
        //     content: pc::Index(),
        // },
        // myw::Tab {
        //     title: "输入法".to_string(),
        //     content: input::Index(),
        // },
        myw::Tab {
            title: "标题".to_string(),
            content: title::Index(),
        },
        myw::Tab {
            title: "链接".to_string(),
            content: link::Index(),
        },
        myw::Tab {
            title: "关于".to_string(),
            content: about::Index(),
        },
    ];

    rsx! {
        div {
            h1 {
                "设置与关于",
            }
            p {
                style: "  word-break: break-all;  /* 关键属性：强制所有字符都可换行 */
                overflow-wrap: anywhere; /* 更智能的断行（兼容性较好） */
                white-space: normal;    /* 允许换行 */
                width: 100%;           /* 必须指定宽度 */
                box-sizing: border-box;",
                "{GLOBAL_USER_AUTH.read()}"
            }
            login::Index{}

            myw::Tabset { tabs }

        }
    }
}
