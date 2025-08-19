use dioxus::prelude::*;

use crate::myw;

// 定义 Modal 组件的 Props

#[component]
pub fn Index() -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div {
            myw::Gap {  }
            myw::Button {
                onclick: move |_| is_open.set(true),
                "开关 Modal"
            }

            myw::Modal {
                is_open,
                title: Some("示例弹窗".to_string()),
                p { "这是一个不使用 Props 结构的 Modal 组件。" }
            }
        }
    }
}
