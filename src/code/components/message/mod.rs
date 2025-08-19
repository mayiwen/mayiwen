use dioxus::prelude::*;

use crate::myw;

// 定义 Modal 组件的 Props

#[component]
pub fn Index() -> Element {
    let mut is_open = use_signal(|| false);
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    rsx! {
           div {
               myw::Gap{}
               myw::Button {
                   onclick: move |_| {

                       messages.write().push(myw::MessageItem {
                           t: myw::MessageType::INFO,
                           m: "你好,这是弹出的消息".to_string(),
                           pin: false,
                       });
                   },
                   "弹出消息"
               }
                myw::Gap{w: "8"}
               myw::Button {
                   onclick: move |_| {

                       messages.write().push(myw::MessageItem {
                           t: myw::MessageType::WARNING,
                           m: "你好,这是警告消息".to_string(),
                           pin: false,
                       });
                   },
                   "警告消息"
               }
    myw::Gap{w: "8"}
               myw::Button {
                   onclick: move |_| {

                       messages.write().push(myw::MessageItem {
                           t: myw::MessageType::ERROR,
                           m: "你好,这是错误消息".to_string(),
                           pin: false,
                       });
                   },
                   "错误消息"
               }

           }
       }
}
