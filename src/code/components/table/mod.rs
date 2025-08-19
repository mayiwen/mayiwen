use std::rc::Rc;

use dioxus::prelude::*;
use web_sys::console;

use crate::myw::{self, TabColumn};
// Example usage with different data types

#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    pub id: usize,
    pub title: String,
    pub status: String,
    pub extra_field1: String,
    pub extra_field2: u32,
    // Can have as many fields as needed
}

// Example usage with test data
// 示例使用
#[derive(Clone, PartialEq)]
struct User {
    id: String,
    name: String,
    age: u32,
    active: bool,
}

#[component]
pub fn ExampleTabset() -> Element {
    // 使用 Signal 来管理 columns 和 data，使其可响应
    let mut columns = use_signal(|| {
        vec![
            TabColumn {
                width: 120,
                title: "序号",
                id: "id",
                renderer: Rc::new(|user: &User| {
                    rsx! {
                        span { "{user.id}" }
                    }
                }),
            },
            TabColumn {
                width: 120,
                title: "年龄",
                id: "age",
                renderer: Rc::new(|user: &User| {
                    rsx! {
                        span { "{user.age}" }
                    }
                }),
            },
            TabColumn {
                width: 200,
                title: "姓名",
                id: "name",
                renderer: Rc::new(|user: &User| {
                    rsx! {
                        span {
                            class: "cell-name",
                            style: "color: var(--primary);",
                            "{user.name}"
                        }
                    }
                }),
            },
            // 其他列...
        ]
    });

    let mut users = use_signal(|| {
        vec![
            User {
                id: "1".to_string(),
                name: "ZS".to_string(),
                age: 18,
                active: true,
            },
            User {
                id: "2".to_string(),
                name: "LS".to_string(),
                age: 19,
                active: true,
            },
            // 其他用户...
        ]
    });

    // 添加按钮来演示响应式更新
    rsx! {
        div {
            style: "padding: 20px;",
            h2 { "myw::Table 用户管理表格示例" }

            myw::Button {
                onclick: move |_| {

                    // // 添加新用户 - 会自动触发表格更新
                    let new_id = users.with(|users| users.len() + 1001);
                          users.write().push(User {
                              id: format!("{}", new_id),
                              name: "New User".to_string(),
                              age: 25,
                              active: true,
                          });
                },
                "添加一个用户"
            }
            myw::Gap{w: "18"}

            myw::Button {
                onclick: move |_| {
                    // 修改列宽 - 会自动触发表格更新
                    columns.write()[0].width += 10;
                },
                "修改id的宽度"
            }

            myw::Table {
                columns: columns,
                data: users,
                row_height: Some(32),
                header_height: Some(32),
                key_fn: |user| user.id.clone(),

            }
        }
    }
}

#[component]
pub fn Index() -> Element {
    rsx! {
        ExampleTabset{}
    }
}
