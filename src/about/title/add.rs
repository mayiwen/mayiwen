
use super::models;
use crate::myw::{self};
use dioxus::prelude::*;
use web_sys;
use web_sys::console;

#[component]
pub fn Add(add_result: EventHandler<bool>) -> Element {
    let mut title = use_signal(|| String::from(""));
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    let show_edit = use_signal(|| false);
    let handle_click = move |_| {
        let title: String = title.read().clone();
        // 如果标题里面没有内容，要给我一个提醒。
        if title.trim().len() == 0 {
            messages.write().push(myw::MessageItem {
                t: myw::MessageType::WARNING,
                m: "需要补充标题的内容".to_string(),
                pin: false,
            });
            return;
        }
        spawn(async move {
            let params = models::TitleAddParams { title };
            let res = super::super::data::fetch_title_add(&params).await;
            match res {
                Ok(res) => {
                    messages.write().push(myw::MessageItem {
                        t: myw::MessageType::INFO,
                        m: "添加成功".to_string(),
                        pin: false,
                    });
                    add_result.call(true);
                }
                Err(error) => {
                    console::log_1(&format!("res: {}", error).into());
                    messages.write().push(myw::MessageItem {
                        t: myw::MessageType::ERROR,
                        m: "添加失败".to_string(),
                        pin: false,
                    });
                }
            }
        });
    };
    rsx! {
        div {
            input {
                class: "myw-input",
                placeholder: "请输入标题",
                value: "{title}",
                oninput: move |e| title.set(e.value()),
            }
            myw::Gap { w: "8" }
            myw::Button { onclick: handle_click, "添加标题" }



        }
    }
}
