use super::data;
use super::models;

use crate::myw::{self};
use crate::GLOBAL_USER_AUTH;

use dioxus::prelude::*;
use web_sys::console;
#[component]
pub fn Index() -> Element {
    let mut is_open = use_signal(|| false);
    let mut username = use_signal(|| String::from(""));
    let mut password = use_signal(|| String::from(""));
    let handle_confirm = move |_| {
        spawn(async move {
            let login = models::Login {
                name: username.read().clone(),
                password: password.read().clone(),
            };
            let res = data::fetch_login(&login).await;
            match res {
                Ok(res) => {
                    if res.access_token == "error".to_string() {
                        let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();

                        messages.write().push(myw::MessageItem {
                            t: myw::MessageType::INFO,
                            m: "马一文才可以登录".to_string(),
                            pin: false,
                        });
                    } else {
                        *GLOBAL_USER_AUTH.write() = res.access_token.clone();
                        console::log_1(&format!("res: {}", res.access_token).into());
                        is_open.set(false);
                    }
                }
                Err(error) => {
                    console::log_1(&format!("res: {}", error).into());
                }
            }
        });
    };

    rsx! {
        myw::Button {
            onclick: move |_| is_open.set(true),
            style: "float: right; margin-right: 10px",
            "马一文登录"
        }
        myw::Modal {
            is_open,
            title: Some("马一文登录".to_string()),
            on_confirm: handle_confirm,
            div { style: "width: 280px;",
                myw::Gap {}
                input {
                    class: "myw-input",
                    placeholder: "请输入密码1",
                    value: "{username}",
                    oninput: move |e| username.set(e.value()),
                }
                myw::Gap { h: "15" }
                input {
                    class: "myw-input",
                    placeholder: "请输入密码2",
                    value: "{password}",
                    oninput: move |e| password.set(e.value()),
                }
                myw::Gap {}
            }
        }
    }
}
