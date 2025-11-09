
use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    static YUEDUQI: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/yueduqi_windows.png");
    static R: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/feature/read.png");
    static S: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/feature/setting.png");
    static B: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/feature/black.png");
    static BG: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/feature/bg.png");
    static C: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/feature/ch.png");
    static D: Asset = asset!("/assets/mayiwen/nuoruo/yueduqi/feature/drag.png");
    rsx! {
        myw::Gap {}
        h3 { "主界面" }
        myw::Gap { h: "8" }
        img { style: "width: 100%; max-width: 400px;", src: YUEDUQI }
        myw::Gap {}
        h3 { "阅读界面" }
        myw::Gap { h: "8" }
        img { style: "width: 100%; max-width: 400px;", src: R }
        myw::Gap {}
        h3 { "设置界面" }
        myw::Gap { h: "8" }
        img { style: "width: 100%; max-width: 400px;", src: S }
        myw::Gap {}
        h3 { "黑色主题" }
        myw::Gap { h: "8" }
        img { style: "width: 100%; max-width: 400px;", src: B }
        myw::Gap {}
        h3 { "自定义背景" }
        myw::Gap { h: "8" }
        img { style: "width: 100%; max-width: 400px;", src: BG }
        myw::Gap {}
        h3 { "获取目录" }
        myw::Gap { h: "8" }
        img { style: "width: 100%; max-width: 400px;", src: C }
        myw::Gap {}
        h3 { "拖拽右下角改变为任意大小 仅桌面端支持" }
        myw::Gap { h: "8" }
        img { style: "width: 100%; max-width: 700px;", src: D }
        myw::Gap {}
    }
}
