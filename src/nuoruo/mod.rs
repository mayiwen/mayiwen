use dioxus::prelude::*;

use crate::myw;

#[component]
pub fn Index() -> Element {
    static NUORUO: Asset = asset!("/assets/mayiwen/nuoruo/nuoruo.gif");
    rsx! {
        div {

            style: "text-align: center",
            myw::Gap {  }
            h1 {
                "诺若电子书阅读器"
            }
            p { "基于rust、tauri、react技术架构。" }
            myw::Gap {  }
            p {
                style: "font-size: 24px",
                "下载地址：",

            }
            p {
                style: "font-size: 18px",
                a {
                    href: "https://gitlink.org.cn/mayiwen/yueduqi/releases",
                    "前往诺若电子书阅读器-windows"
                }

            }
            p {
                style: "font-size: 18px",
                "页面下载exe文件",

            }
            // p {
            //     style: "font-size: 18px",

            //     a {
            //         href: "https://gitlink.org.cn/mayiwen/yueduqi",
            //         "访问gitlink仓库"
            //     }

            // }

             myw::Gap {  }

            img {
                style: "width: 100%; max-width: 1000px;",
                src: NUORUO,
                alt: "马一文桌面端截图"
            }
            p {   "以诺若的名义发布面向应用商店的作品。以后将迁移到nuoruo.com" }

        }
    }
}
