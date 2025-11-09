use dioxus::prelude::*;

use crate::myw;

mod android;
mod ios;
mod linux;
mod macos;
mod show;
mod windows;

#[component]
pub fn Index() -> Element {
    let tabs = vec![
        myw::Tab {
            title: "ios".to_string(),
            content: ios::Index(),
        },
        myw::Tab {
            title: "android".to_string(),
            content: android::Index(),
        },
        myw::Tab {
            title: "windows".to_string(),
            content: windows::Index(),
        },
        myw::Tab {
            title: "macos".to_string(),
            content: macos::Index(),
        },
        myw::Tab {
            title: "linux".to_string(),
            content: linux::Index(),
        },
        myw::Tab {
            title: "功能".to_string(),
            content: show::Index(),
        },
    ];
    rsx! {
        div { style: "text-align: center",

            p { style: "text-align: right; margin-right: 8px;",
                a {
                    href: "https://gitlink.org.cn/mayiwen/yueduqi",
                    target: "_blank",
                    title: "gitlink 访问源码",

                    myw::Button { style: "padding-top: 3px!important;", myw::icon::GitLink {} }
                }
                myw::Gap { w: "8" }
                a {
                    href: "https://github.com/mayiwen/yueduqi",
                    target: "_blank",
                    title: "github 访问源码",
                    myw::Button { myw::icon::Github {} }
                }
            }

            h1 {
               title: "https://mayiwen.com/yueduqi",
                "一文小说阅读器" }
            myw::Gap {}

            a {
                href: "https://gitlink.org.cn/mayiwen/yueduqi/releases",
                title: "https://gitlink.org.cn/mayiwen/yueduqi/releases",
                target: "_blank",
                myw::Button { style: "transform: scale(1.4)", "gitlink下载开源软件" }
            }
            myw::Gap { h: "20" }
            p { a {
                href: "https://github.com/mayiwen/yueduqi/releases",
                title: "https://github.com/mayiwen/yueduqi/releases",
                target: "_blank",
                "github分流下载"
            }}
            myw::Gap { h: "20" }
            div { style: "width: 375px; margin: auto; text-align: left",
                myw::Tabset { tabs }
            }
        }
    }
}
