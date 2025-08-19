use std::fmt::Display;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use super::models;
use crate::myw::{self, Tab, TabColumn};
use dioxus::events::KeyboardEvent;
use dioxus::html::h1;
use dioxus::{html::view, prelude::*};
use keyboard_types::Key;
use serde::{self, Deserialize, Serialize};
use web_sys;
use web_sys::console;
mod add;
mod del;
mod edit;

#[component]
fn ActionColumn(show_edit: Signal<bool>) -> Element {
    rsx! {
        myw::Button {
            onclick: move |_| show_edit.set(true),
            "修改"
        }
        myw::Gap{w: "8"}
        myw::Button { "删除" }
    }
}
pub fn NavTable(mut props: models::NavTableProps) -> Element {
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    let default_id = props.items.first().map(|item| item.id).unwrap_or(0);
    let mut selected_id = use_signal(|| default_id);

    let on_tab_change = move |tab_id: u32| {
        println!("Tab changed to: {}", tab_id);
        selected_id.set(tab_id);
    };
    let mut show_edit = use_signal(|| false);
    let mut show_del = use_signal(|| false);
    let edit_title = use_signal(|| models::NavTitle {
        id: 0,
        title: "this is title ".to_string(),
        index: 0,
    });

    let del_title = use_signal(|| models::NavTitle {
        id: 0,
        title: "示例文字".to_string(),
        index: 0,
    });

    let mut columns = use_signal(|| {
        // let show_edit = show_edit.clone();
        vec![
            TabColumn {
                width: 120,
                title: "序号",
                id: "id",
                renderer: Rc::new(|title: &models::NavTitle| {
                    rsx! {
                        span { "{title.id}" }
                    }
                }),
            },
            TabColumn {
                width: 200,
                title: "标题",
                id: "title",
                renderer: Rc::new(|title: &models::NavTitle| {
                    rsx! {
                        span { "{title.title}" }
                    }
                }),
            },
            TabColumn {
                width: 200,
                title: "下标",
                id: "index",
                renderer: Rc::new(|title: &models::NavTitle| {
                    rsx! {
                        span { "{title.index}" }
                    }
                }),
            },
            TabColumn {
                width: 90,
                title: "操作",
                id: "edit",
                // renderer: Rc::new(move |_| rsx! { ActionColumn { show_edit: show_edit.clone() } }),
                renderer: {
                    let mut show_edit = show_edit.clone();
                    let mut show_del = show_del.clone();
                    let mut edit = edit_title.clone();
                    let mut del = del_title.clone();
                    Rc::new(move |title: &models::NavTitle| {
                        let title_clone = title.clone();
                        let del_clone = title.clone();
                        rsx! {
                            myw::Button {
                                onclick: move |_| {
                                    show_edit.set(true);
                                    edit.set(title_clone.clone());
                                },
                                "修改"
                            }
                            myw::Gap{w: "8"}
                            myw::Button {
                                onclick: move |_| {
                                    show_del.set(true);
                                    del.set(del_clone.clone());
                                },
                                "删除"
                            }
                        }
                    })
                },
            },
        ]
    });

    let handle_row_moved: Callback<(u32, u32)> =
        Callback::new(move |(from_idx, to_idx): (u32, u32)| {
            use web_sys::console;
            println!("Row moved from {} to {}", from_idx, to_idx);
            console::log_1(&format!("Row moved from {} to {}", from_idx, to_idx).into());

            // 记录消息
            messages.write().push(myw::MessageItem {
                t: myw::MessageType::INFO,
                m: format!("{}-{}", from_idx, to_idx),
                pin: false,
            });

            // 获取当前id值（同步部分）
            // let current_id = *id.read();

            // // 修改nav_link数据（使用 with_mut 确保线程安全）
            // nav_link.write().with_mut(|links| {
            //     let from_idx = from_idx as usize;
            //     let to_idx = to_idx as usize;

            //     // 边界检查
            //     if from_idx < links.len() && to_idx <= links.len() {
            let mut nav_title_one = props.items.read().clone();
            let item_save = nav_title_one.remove(from_idx as usize);
            nav_title_one.insert(to_idx as usize, item_save);
            //     }
            // });

            // 异步更新数据
            spawn(async move {
                // 准备要更新的数据
                let navlist = nav_title_one
                    .iter()
                    .enumerate()
                    .map(|(index, x)| models::NavTitleSort {
                        id: x.id,
                        // title: x.title.clone(),
                        // src: x.src.clone(),
                        // titleId: x.title_id.clone(),
                        index: index as u32,
                    })
                    .collect::<Vec<_>>();

                // 更新服务器数据
                let _ = super::data::fetch_nav_title_update_list(&navlist).await;

                // 重新从服务器获取最新数据
                // super::data::fetch_nav_link(current_id).await;
                props.items.set(vec![]);

                let links = super::data::fetch_nav_titles().await;
                props.items.set(links);
            });
        });

    rsx! {
        div {
            style: "",
            myw::Gap{}
            add::Add {
                add_result: EventHandler::new(move |result: bool| {
                    println!("Received result: {}", result);
                    props.add_result.call(true);
                }),
            }
            myw::Table {
                columns: columns,
                data: props.items,
                row_height: Some(32),
                header_height: Some(32),
                key_fn: |title| title.id.to_string(),
                 on_row_index_change : handle_row_moved,
            }
        }


        edit::Edit {
            show_edit: show_edit,
            edit: edit_title,
            edit_result: EventHandler::new(move |result: bool| {
                println!("Received result: {}", result);
                  props.add_result.call(true);
                  show_edit.set(false);
            })
        }

        del::Index {
            show: show_del,
            ctrl: del_title,
            ctrl_result: EventHandler::new(move |result: bool| {
                println!("Received result: {}", result);
                show_del.set(false);
                props.add_result.call(true);
            })
        }

    }
}

#[component]
pub fn Index() -> Element {
    let mut nav_title = use_signal(|| Vec::<models::NavTitle>::new());
    let count = use_signal(|| 1);

    // let mut data = use_resource(move || async { super::backend::fetch_nav_titles().await });
    use_effect(move || {
        // let current_id = *count.read(); // 显式读取 id 值

        spawn(async move {
            nav_title.set(vec![]);
            let titles = super::data::fetch_nav_titles().await;
            nav_title.set(titles);
        });
    });
    rsx! {
        NavTable { items: nav_title, add_result: EventHandler::new(move |result: bool| {
            println!("Received result: {}", result);
                // 可以在这里修改全局状态、导航等
                // set_version.(version() + 1);  // 修改版本触发重新加载
                // count.set(*count.read() + 1); // 正确修改 count
                spawn(async move {
                    nav_title.set(vec![]);
                    let titles = super::data::fetch_nav_titles().await;
                    nav_title.set(titles);
                });
        }), }
    }
}
