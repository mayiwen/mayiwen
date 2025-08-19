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
    rsx! {
        div {
            style: "text-align: center",
            h1 {
                "马一文的输入法",
            },
            DownloadLinks{},
        }
    }
}

#[component]
pub fn DownloadLinks() -> Element {
    static ANDROID: Asset = asset!("/assets/mayiwen/input-method/android-rime.jpg");
    static LINUX: Asset = asset!("/assets/mayiwen/input-method/linux-rime.png");
    static WINDOWS: Asset = asset!("/assets/mayiwen/input-method/windows-duoduo.png");
    rsx! {
        div {
            div {
                style: "text-align: center",

                myw::Gap {  }  // 自定义组件需要单独定义

                p {
                    a {
                        href: "https://gitlink.org.cn/mayiwen/input/releases",
                        target: "_blank",
                        style: "font-size: 24px;",
                        "前往下载"
                    }
                }

                h3 { "windows、android提供安装包，其他平台，提供通用方案。" }

                p {
                    "下载方案后，可部署到rime输入法平台(小狼毫windows、鼠鬚管mac、中州韻linux、同文输入法android)以实现多端兼容"
                }

                p {}

                img {
                    style: "width: 100%; max-width: 375px;",
                    src: ANDROID
                }

                p {}

                img {
                    style: "width: 100%; max-width: 375px;",
                    src: LINUX
                }

                p {}

                img {
                    style: "width: 100%; max-width:375px;",
                    src: WINDOWS
                }
            }
        }
    }
}
