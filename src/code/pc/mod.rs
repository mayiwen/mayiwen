
use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    static MAYIWEN: Asset = asset!("/assets/mayiwen/pc/mayiwen.png");
    static BROWSER: Asset = asset!("/assets/mayiwen/pc/browser.png");
    rsx! {
        div { style: "text-align: center",

            h1 { "马一文桌面端" }
            myw::Gap { h: "20" }
            p {
                a {
                    href: "https://gitlink.org.cn/mayiwen/nuoruo/releases",
                    target: "_blank",
                    myw::Button { style: "transform: scale(1.4)", "获取开源软件" }
                }
            }
            myw::Gap { h: "20" }
            img {
                style: "width: 100%; max-width: 1000px;",
                src: MAYIWEN,
                alt: "马一文桌面端截图",
            }

            img {
                style: "width: 100%; max-width: 1000px;",
                src: BROWSER,
                alt: "浏览器截图",
            }

            p { "0.4 版本在使用rust tauri 重置，现此版本已废弃" }
            p { "构建一个浏览器为目标，构建一个个人用多端软件" }
        }
    }
}
