
use crate::myw::{self};
use dioxus::prelude::*;


#[component]
pub fn Index() -> Element {
    rsx! {
        div { style: "text-align: center",
            h1 { "马一文的输入法" }
            DownloadLinks {}
        }
    }
}

#[component]
pub fn DownloadLinks() -> Element {
    
    let tabs = vec![
        myw::Tab {
            title: "ios".to_string(),
            content: Ios(),
        },
        myw::Tab {
            title: "android".to_string(),
            content: Android(),
        },
        myw::Tab {
            title: "windows".to_string(),
            content: Windows(),
        },
        myw::Tab {
            title: "macos".to_string(),
            content: Macos(),
        },
        myw::Tab {
            title: "linux".to_string(),
            content: Linux(),
        },
    ];

    rsx! {
        div {
            myw::Gap { h: "20" }
            p {
                p {
                    a {
                        href: "https://gitlink.org.cn/mayiwen/input/releases",
                        target: "_blank",
                        myw::Button { style: "transform: scale(1.4)", "前往下载" }
                    }
                }
            }
            myw::Gap { h: "20" }
            h3 { "windows、android提供安装包" }
            p { "windows 基于多多输入法打包" }
            p { "android 基于同文输入法打包" }
            h3 { "其他平台，提供通用方案。" }
            p {
                "下载方案后，可部署到rime输入法平台以实现多端兼容"
            }
        
            p{
                "下面是rime输入法在各平台的输入法名称。"
            }
            p {"windows 小狼毫输入法"}
            p {"mac 鼠鬚管输入法"}
            p {"linux 中州韻输入法"}
            p {"andorid 同文输入法"}
            p {"ios 仓输入法"}
            myw::Gap { h: "20" }
            p {"ios 安卓提供自用输入法主题"}
            div { style: "width: 375px; margin: auto; text-align: left",
                myw::Tabset { tabs }
            }
        }
    }
}


#[component]
pub fn Android() -> Element {
    static ANDROID: Asset = asset!("/assets/mayiwen/input-method/android-rime.jpg");
    rsx! {
        myw::Gap {}
        h3 { "android " }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: ANDROID,
            alt: "android",
        }
    }
}
#[component]
pub fn Ios() -> Element {
    static IOS: Asset = asset!("/assets/mayiwen/input-method/ios-input.jpg");
    rsx! {
        myw::Gap {}
        h3 { "ios " }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: IOS,
            alt: "iso",
        }
    }
}

#[component]
pub fn Windows() -> Element {
    static WINDOWS: Asset = asset!("/assets/mayiwen/input-method/windows-duoduo.png");
    rsx! {
        myw::Gap {}
        h3 { "windows " }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: WINDOWS,
            alt: "windows",
        }
    }
}

#[component]
pub fn Macos() -> Element {
    static MACOS: Asset = asset!("/assets/mayiwen/input-method/macos.png");
    rsx! {
        myw::Gap {}
        h3 { "macos " }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: MACOS,
            alt: "macos",
        }
    }
}

#[component]
pub fn Linux() -> Element {
    static LINUX: Asset = asset!("/assets/mayiwen/input-method/linux-rime.png");
    rsx! {
        myw::Gap {}
        h3 { "linux " }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: LINUX,
            alt: "linux",
        }
    }
}