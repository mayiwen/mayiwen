
use super::models;
use crate::myw::{self};
use dioxus::prelude::*;
// pub items: Signal<Vec<NavTitle>>,
// pub add_result: EventHandler<bool>,
#[component]
pub fn Index(
    show: Signal<bool>,
    ctrl: Signal<models::NavTitle>,
    ctrl_result: EventHandler<bool>,
) -> Element {
    let mut id: Signal<u32> = use_signal(|| 0);
    let mut title = use_signal(|| "你好".to_string());
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    // 当 edit 变化时，同步更新 id 和 title
    use_effect(move || {
        id.set(ctrl().id);
        title.set(ctrl().title.clone());
    });

    let handle_confirm = move |_| {
        spawn(async move {
            let id = id.read().clone();
            let res = super::super::data::fetch_title_del(id).await;
            if res {
                messages.write().push(myw::MessageItem {
                    t: myw::MessageType::INFO,
                    m: "删除成功".to_string(),
                    pin: false,
                });
                ctrl_result.call(true);
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
            title: Some("是否确认删除标题？".to_string()),
            on_confirm: handle_confirm,
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
                    disabled: true,
                    oninput: move |event| {
                         title.set(event.value())

                    }
                }
                myw::Gap{},
            }
        }
    }
}
