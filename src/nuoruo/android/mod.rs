
use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    static NUORUO_READER: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/yueduqi_apk.jpg");
    rsx! {
        myw::Gap {}
        h3 { "andorid 主界面" }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: NUORUO_READER,
            alt: "andorid",
        }
        h4 { "截图来源" }
        p { "系统：andorid 12" }
        p { "CPU架构：ARM" }
        p { "设备信息：Redmi Note 9 Pro" }
    }
}
