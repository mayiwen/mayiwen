
use super::models;
use crate::myw::{self};
use dioxus::prelude::*;
// pub items: Signal<Vec<NavTitle>>,
// pub add_result: EventHandler<bool>,
#[component]
pub fn Edit(
    show_edit: Signal<bool>,
    edit: Signal<models::NavTitle>,
    edit_result: EventHandler<bool>,
) -> Element {
    let mut id: Signal<u32> = use_signal(|| 0);
    let mut index: Signal<i32> = use_signal(|| 0);
    let mut title = use_signal(|| "你好".to_string());
    // 当 edit 变化时，同步更新 id 和 title
    use_effect(move || {
        id.set(edit().id);
        title.set(edit().title.clone());
        index.set(edit().index.clone());
    });
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    let handle_confirm = move |_| {
        spawn(async move {
            let id = id.read().clone();
            let title = title.read().clone();
            let index = index.read().clone();
            let params = models::NavTitle { id, title, index };

            let res = super::super::data::fetch_title_update(&params).await;
            if res {
                messages.write().push(myw::MessageItem {
                    t: myw::MessageType::INFO,
                    m: "修改成功".to_string(),
                    pin: false,
                });
                edit_result.call(true);
            } else {
                messages.write().push(myw::MessageItem {
                    t: myw::MessageType::INFO,
                    m: "修改失败".to_string(),
                    pin: false,
                });
            }
        });
    };
    rsx! {
        myw::Modal {
            is_open: show_edit,
            title: Some("修改标题".to_string()),
            on_confirm:handle_confirm,
            div {
                style: "width: 280px;",
                myw::Gap{},
                input {
                    class: "myw-input",
                    r#type: "number",
                    placeholder: "Enter ID",
                    value: "{id}",
                    disabled: true,
                    oninput: move |e| {
                        if let Ok(num) = e.value().parse::<u32>() {
                            id.set(num);
                        }
                    }
                }
                myw::Gap{h: "15"},
                input {
                    class: "myw-input",
                    r#type: "text",
                    placeholder: "Enter ID",
                    value: "{title}",
                    oninput: move |event| {
                         title.set(event.value())

                    }
                }
                myw::Gap{},
            }
        }
    }
}
