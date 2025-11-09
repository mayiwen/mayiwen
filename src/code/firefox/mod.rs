use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    static FIREFOX: Asset = asset!("/assets/mayiwen/firefox/firefox.png");
    rsx! {
        div { style: "text-align: center",

            h1 { "火狐插件  mayiwen" }
            p { "可以火狐插件中搜索“mayiwen”找到此插件。" }
            myw::Gap { h: "20" }
            p {
                p {
                    a {
                        href: "https://addons.mozilla.org/zh-CN/firefox/addon/mayiwen/",
                        target: "_blank",
                        myw::Button { style: "transform: scale(1.4)", "获取开源软件" }
                    }
                }
            }
            myw::Gap {}
            p { "实现的功能：" }
            p {
                "1. 点击插件的图标，可以关闭其他标签页、关闭左侧标签页、关闭右侧标签页。"
            }
            p { "2. 在标签处右键，出现关闭其他标签选项。" }
            p { "3. 在网页右键，出现关闭其他标签选项。" }
            p { "4. 使用快捷键 CTRL + Q，关闭其他标签。" }
            myw::Gap {}
            img {
                style: "width: 100%; max-width: 1000px;",
                src: FIREFOX,
                alt: "firefox 插件截图",
            }

        }
    }
}
