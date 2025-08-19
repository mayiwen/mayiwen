use std::fmt::Display;
use std::rc::Rc;

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
pub fn Index() -> Element {
    // 1. 移除了 mut，使用正确的类型声明
    let mut nav_titles = use_signal(|| Vec::<myw::TabTitle>::new());
    let mut id: Signal<u32> = use_signal(|| 1);
    // let tabs: Vec<myw::TabTitle> = items
    //     .iter()
    //     .map(|item| myw::TabTitle {
    //         title: item.title.clone(),
    //         id: item.id,
    //     })
    //     .collect();
    // 2. 使用 use_effect 替代直接 spawn 确保生命周期安全
    use_effect(move || {
        spawn(async move {
            let titles = super::data::fetch_nav_titles().await;
            let titles: Vec<myw::TabTitle> = titles
                .iter()
                .map(|item| myw::TabTitle {
                    title: item.title.clone(),
                    id: item.id,
                })
                .collect();
            let id_init = if titles.len() > 0 { titles[0].id } else { 1 };
            nav_titles.set(titles);
            id.set(id_init);
        });
    });
    let on_tab_change = move |tab_id: u32| {
        println!("Tab changed to: {}", tab_id);
        // selected_id.set(tab_id);
    };
    rsx! {
        div {
            h2 { "链接设置" }

            // 3. 添加加载状态处理
            myw::Tabs {
                tabs: nav_titles,
                on_tab_change,
                active_id: id
            },
            Link {id}

        }
    }
}
#[component]
pub fn Link(id: Signal<u32>) -> Element {
    // // 1. 移除了 mut，使用正确的类型声明
    let mut messages = consume_context::<Signal<Vec<myw::MessageItem>>>();
    let mut nav_link = use_signal(|| Vec::<models::NavLink>::new());
    let mut show_edit = use_signal(|| false);
    let mut show_del = use_signal(|| false);
    let edit = use_signal(|| models::NavLink {
        id: 0,
        title: "".to_string(),
        src: "".to_string(),
        title_id: 0,
        index: 0,
    });

    let del = use_signal(|| models::NavLink {
        id: 0,
        title: "".to_string(),
        src: "".to_string(),
        title_id: 0,
        index: 0,
    });
    let mut columns = use_signal(|| {
        vec![
            TabColumn {
                width: 120,
                title: "序号",
                id: "id",
                renderer: Rc::new(|title: &models::NavLink| {
                    rsx! {
                        span { "{title.id}" }
                    }
                }),
            },
            TabColumn {
                width: 150,
                title: "链接标题",
                id: "title",
                renderer: Rc::new(|title: &models::NavLink| {
                    rsx! {
                        span { "{title.title}" }
                    }
                }),
            },
            TabColumn {
                width: 300,
                title: "链接",
                id: "src",
                renderer: Rc::new(|title: &models::NavLink| {
                    rsx! { span { "{title.src}" } }
                }),
            },
            TabColumn {
                width: 100,
                title: "排序",
                id: "index_link",
                renderer: Rc::new(|title: &models::NavLink| {
                    rsx! { span { "{title.index}" } }
                }),
            },
            TabColumn {
                width: 50,
                title: "类别",
                id: "title_id",
                renderer: Rc::new(|title: &models::NavLink| {
                    rsx! {
                        span { "{title.title_id}" }
                    }
                }),
            },
            TabColumn {
                width: 90,
                title: "操作",
                id: "edit",
                renderer: {
                    let mut show_edit = show_edit.clone();
                    let mut show_del = show_del.clone();
                    let mut edit = edit.clone();
                    let mut del = del.clone();
                    Rc::new(move |link: &models::NavLink| {
                        let add_clone = link.clone();
                        let del_clone = link.clone();
                        rsx! {
                            myw::Button {
                                onclick: move |_| {
                                    show_edit.set(true);
                                    edit.set(add_clone.clone());
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
            // 其他列...
        ]
    });

    // let columns = use_signal(|| columns);
    use_effect(move || {
        let current_id = *id.read(); // 显式读取 id 值

        spawn(async move {
            nav_link.set(vec![]);
            let links = super::data::fetch_nav_link(current_id).await;
            nav_link.set(links);
        });
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
            let current_id = *id.read();

            // // 修改nav_link数据（使用 with_mut 确保线程安全）
            // nav_link.write().with_mut(|links| {
            //     let from_idx = from_idx as usize;
            //     let to_idx = to_idx as usize;

            //     // 边界检查
            //     if from_idx < links.len() && to_idx <= links.len() {
            let mut nav_link_one = nav_link.read().clone();
            let item_save = nav_link_one.remove(from_idx as usize);
            nav_link_one.insert(to_idx as usize, item_save);
            //     }
            // });

            // 异步更新数据
            spawn(async move {
                // 准备要更新的数据
                let navlist = nav_link_one
                    .iter()
                    .enumerate()
                    .map(|(index, x)| models::NavLinkSort {
                        id: x.id,
                        // title: x.title.clone(),
                        // src: x.src.clone(),
                        // titleId: x.title_id.clone(),
                        index: index as u32,
                    })
                    .collect::<Vec<_>>();

                // 更新服务器数据
                let _ = super::data::fetch_nav_link_update_list(&navlist).await;

                // 重新从服务器获取最新数据
                // super::data::fetch_nav_link(current_id).await;
                nav_link.set(vec![]);

                let links = super::data::fetch_nav_link(current_id).await;
                nav_link.set(links);
            });
        });
    rsx! {
        div {
            style: "width: 100%; padding: 2px",

            // myw::Button {"添加链接"}
            add::Index{id, ctrl_result: EventHandler::new(move |result: bool| {
                println!("Received result: {}", result);
                // props.add_result.call(true);
                spawn(async move {
                    let current_id = *id.read(); // 显式读取 id 值
                    nav_link.set(vec![]);
                    let links = super::data::fetch_nav_link(current_id).await;
                    nav_link.set(links);
                });
            }),}

            myw::Table {
                columns: columns,
                data: nav_link,
                row_height: Some(32),
                header_height: Some(32),
                key_fn: |title| title.id.to_string(),
                on_row_index_change : handle_row_moved,
            }
            edit::Index {
                show: show_edit,
                data: edit,
                ctrl: EventHandler::new(move |result: bool| {
                    println!("Received result: {}", result);
                    // props.add_result.call(true);
                    spawn(async move {
                        let current_id = *id.read(); // 显式读取 id 值
                        nav_link.set(vec![]);
                        show_edit.set(false);
                        let links = super::data::fetch_nav_link(current_id).await;
                        nav_link.set(links);

                    });
                })
            }
            del::Index {
                show: show_del,
                data: del,
                ctrl: EventHandler::new(move |result: bool| {
                    println!("Received result: {}", result);
                    // props.add_result.call(true);
                    spawn(async move {
                        let current_id = *id.read(); // 显式读取 id 值
                        nav_link.set(vec![]);
                        show_del.set(false);
                        let links = super::data::fetch_nav_link(current_id).await;
                        nav_link.set(links);

                    });
                })
            }


        }
    }
}
