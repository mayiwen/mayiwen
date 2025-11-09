
use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    static NUORUO_READER_IOS: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/yueduqi_ios.jpg");
    rsx! {
        myw::Gap {}
        h3 { "ios 主界面" }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: NUORUO_READER_IOS,
            alt: "iso",
        }
        h4 { "截图来源" }
        p { "系统：ios 26" }
        p { "CPU架构：Apple" }
        p { "设备信息：iPhone 17 pro max" }
    }
}
