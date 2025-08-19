use std::fmt::Display;

use crate::myw::{self, Tab};
use dioxus::events::KeyboardEvent;
use dioxus::html::h1;
use dioxus::{html::view, prelude::*};
use keyboard_types::Key;
use serde::{self, Deserialize, Serialize};
use web_sys;

#[component]
pub fn Index() -> Element {
    static MAYIWEN: Asset = asset!("/assets/mayiwen/pc/mayiwen.png");
    static BROWSER: Asset = asset!("/assets/mayiwen/pc/browser.png");
    rsx! {
        div {
            style: "text-align: center",

            h1 { "马一文桌面端" }

            p {
                a {
                    href: "https://gitlink.org.cn/mayiwen/nuoruo/releases",
                    target: "_blank",
                    style: "font-size: 24px;",
                    "windows版下载"
                }
            }

            img {
                style: "width: 100%; max-width: 1000px;",
                src: MAYIWEN,
                alt: "马一文桌面端截图"
            }

            img {
                style: "width: 100%; max-width: 1000px;",
                src: BROWSER,
                alt: "浏览器截图"
            }
        }
    }
}
