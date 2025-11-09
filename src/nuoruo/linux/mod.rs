
use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    static NUORUO_READER: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/yueduqi_deb.png");
    rsx! {
        myw::Gap {}
        h3 { "linux 主界面" }
        myw::Gap {}
        img {
            style: "width: 100%; max-width: 400px;",
            src: NUORUO_READER,
            alt: "linux",
        }
        h4 { "截图来源" }
        p { "系统：Ubuntu 25.04 64位" }
        p { "CPU架构：AMD64" }
        p { "设备信息：组装机 Intel(R) Xeon(R) CPU E5-1650 v2" }
    }
}
