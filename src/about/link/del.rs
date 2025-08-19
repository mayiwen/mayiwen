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
// pub items: Signal<Vec<NavTitle>>,
// pub add_result: EventHandler<bool>,
#[component]
pub fn Index(
    show: Signal<bool>,
    data: Signal<models::NavLink>,
    ctrl: EventHandler<bool>,
) -> Element {
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    let mut id: Signal<u32> = use_signal(|| 0);
    let mut title = use_signal(|| "".to_string());
    let mut src = use_signal(|| "".to_string());
    let mut title_id = use_signal(|| 0);
    let mut index_link = use_signal(|| 0);
    use_effect(move || {
        id.set(data().id);
        title.set(data().title.clone());
        src.set(data().src.clone());
        title_id.set(data().title_id);
        index_link.set(data().index)
    });
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    let handle_confirm = move |_| {
        spawn(async move {
            let id = id.read().clone();
            let title = title.read().clone();
            let src = src.read().clone();
            let title_id = title_id.read().clone();
            let index_link = index_link.read().clone();
            let params = models::NavLinkAdd {
                index: index_link,
                src,
                title,
                title_id: title_id,
            };

            let res = super::super::data::fetch_link_del(id).await;
            if res {
                messages.write().push(myw::MessageItem {
                    t: myw::MessageType::INFO,
                    m: "删除成功".to_string(),
                    pin: false,
                });
                ctrl.call(true);
            } else {
                messages.write().push(myw::MessageItem {
                    t: myw::MessageType::INFO,
                    m: "删除失败".to_string(),
                    pin: false,
                });
            }
        });
    };
    rsx! {
        myw::Modal {
            is_open: show,
            title: Some("确认删除链接？".to_string()),
            on_confirm: handle_confirm,
            div {
                style: "width: 280px;",
                myw::Gap{},
                div {
                    div {
                        style: "display:inline-block; width: 80px",
                        "序号"
                    }
                    input {
                        class: "myw-input",
                        r#type: "number",
                        placeholder: "序号",
                        value: "{id}",
                        disabled: true,
                        oninput: move |e| {
                            if let Ok(num) = e.value().parse::<u32>() {
                                id.set(num);
                            }
                        }
                    }
                }
                myw::Gap{h: "15"},

                div {
                    div {
                        style: "display:inline-block; width: 80px",
                        "标题"
                    }
                    input {
                        class: "myw-input",
                        r#type: "text",
                        placeholder: "标题",
                        value: "{title}",
                        oninput: move |event| {
                             title.set(event.value())
                        },
                         disabled: true,
                    }
                }


                myw::Gap{h: "15"},

                div {
                    div {
                        style: "display:inline-block; width: 80px",
                        "链接"
                    }
                    input {
                        class: "myw-input",
                        r#type: "text",
                        placeholder: "链接",
                        value: "{src}",
                        oninput: move |event| {
                             src.set(event.value())

                        },
                         disabled: true,
                    }
                }

                myw::Gap{h: "15"},
                div {
                    div {
                        style: "display:inline-block; width: 80px",
                        "标题Id"
                    }
                    input {
                        class: "myw-input",
                        r#type: "text",
                        placeholder: "标题Id",
                        value: "{title_id}",
                        oninput: move |e| {
                            if let Ok(num) = e.value().parse::<u32>() {
                                id.set(num);
                            }
                        },
                         disabled: true,
                    }
                }

                myw::Gap{h: "15"},
            }
        }
    }
}
