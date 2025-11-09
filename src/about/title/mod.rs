use std::rc::Rc;

use super::models;
use crate::myw::{self, TabColumn};
use dioxus::prelude::*;
use web_sys;
mod add;
mod del;
mod edit;

#[component]
fn ActionColumn(show_edit: Signal<bool>) -> Element {
    rsx! {
        myw::Button { onclick: move |_| show_edit.set(true), "修改" }
        myw::Gap { w: "8" }
        myw::Button { "删除" }
    }
}
pub fn NavTable(mut props: models::NavTableProps) -> Element {
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();

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

    let columns = use_signal(|| {
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
                width: 120,
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
                            span {
                                style: "display: inline-block; margin-top: 2px",
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
        div { style: "",
            myw::Gap { h: "8" }
            h2 { "首页标题设置" }
            myw::Gap { h: "8" }
            add::Add {
                add_result: EventHandler::new(move |result: bool| {
                    println!("Received result: {}", result);
                    props.add_result.call(true);
                }),
            }
            myw::Gap { h: "8" }
            myw::Table {
                columns,
                data: props.items,
                row_height: Some(40),
                header_height: Some(40),
                key_fn: |title| title.id.to_string(),
                on_row_index_change: handle_row_moved,
            }
        }


        edit::Edit {
            show_edit,
            edit: edit_title,
            edit_result: EventHandler::new(move |result: bool| {
                println!("Received result: {}", result);
                props.add_result.call(true);
                show_edit.set(false);
            }),
        }

        del::Index {
            show: show_del,
            ctrl: del_title,
            ctrl_result: EventHandler::new(move |result: bool| {
                println!("Received result: {}", result);
                show_del.set(false);
                props.add_result.call(true);
            }),
        }
    }
}

#[component]
pub fn Index() -> Element {
    let mut nav_title = use_signal(|| Vec::<models::NavTitle>::new());

    use_effect(move || {
        spawn(async move {
            nav_title.set(vec![]);
            let titles = super::data::fetch_nav_titles().await;
            nav_title.set(titles);
        });
    });
    rsx! {
        NavTable {
            items: nav_title,
            add_result: EventHandler::new(move |result: bool| {
                println!("Received result: {}", result);
                spawn(async move {
                    nav_title.set(vec![]);
                    let titles = super::data::fetch_nav_titles().await;
                    nav_title.set(titles);
                });
            }),
        }
    }
}
