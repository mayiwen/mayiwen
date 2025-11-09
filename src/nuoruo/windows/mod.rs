
use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    static YUEDUQI: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/yueduqi_windows.png");
    rsx! {
        myw::Gap {}
        h3 { "windows 主界面" }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: YUEDUQI,
            alt: "iso",
        }
        h4 { "截图来源" }
        p { "系统：windows 11 64位" }
        p { "CPU架构：AMD64" }
        p { "设备信息：惠普暗夜精灵8" }
    }
}
