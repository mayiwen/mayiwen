use std::fmt::Display;

use super::models;
use crate::myw::{self, Tab, TabColumn};
use dioxus::events::KeyboardEvent;
use dioxus::html::h1;
use dioxus::{html::view, prelude::*};
use keyboard_types::Key;
use serde::{self, Deserialize, Serialize};
use web_sys;
use web_sys::console;

#[component]
// ,
pub fn Index(id: Signal<u32>, ctrl_result: EventHandler<bool>) -> Element {
    let mut title_id: Signal<u32> = use_signal(|| 0);
    let mut link_title = use_signal(|| "".to_string());
    let mut link_src = use_signal(|| "".to_string());
    // 当 edit 变化时，同步更新 id 和 title
    use_effect(move || {
        title_id.set(id());
    });
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    let handle_confirm = move |_| {
        spawn(async move {
            let title_id = title_id.read().clone();
            let link_title = link_title.read().clone();
            let link_src = link_src.read().clone();
            let params = models::NavLinkAdd {
                title_id: title_id,
                index: 9_9999_9999,
                src: link_src,
                title: link_title,
            };

            let res = super::super::data::fetch_link_add(&params).await;
            match res {
                Ok(res) => {
                    messages.write().push(myw::MessageItem {
                        t: myw::MessageType::INFO,
                        m: "添加成功".to_string(),
                        pin: false,
                    });
                    ctrl_result.call(true);
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

                myw::Gap{h: "8"},
                input {
                    class: "myw-input",
                    r#type: "number",
                    placeholder: "ID",
                    value: "{title_id}",
                    disabled: true,
                    oninput: move |e| {
                        if let Ok(num) = e.value().parse::<u32>() {
                            title_id.set(num);
                        }
                    }
                }
                myw::Gap{w: "8"},
                input {
                    class: "myw-input",
                    r#type: "text",
                    placeholder: "请输入标题",
                    value: "{link_title}",
                    oninput: move |event| {
                         link_title.set(event.value())

                    }
                }
                 myw::Gap{w: "8"},
                 input {
                    class: "myw-input",
                    r#type: "text",
                    placeholder: "请输入链接",
                    value: "{link_src}",
                    oninput: move |event| {
                          link_src.set(event.value())
                    }
                }
                myw::Gap{w: "8"},
                myw::Button {
                    onclick: handle_confirm,
                    "添加"
                }
            }
    }
}
