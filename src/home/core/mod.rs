use std::fmt::Display;

use crate::myw::{self};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;
use keyboard_types::Key;
use serde::{self, Deserialize, Serialize};
use web_sys;
use web_sys::console;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div { style: "width: 100%; text-align: center;",
            a { href: "http://mayiwen.com", target: "_blank", // Opens in new tab
                img {
                    src: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMgAAABLAgMAAABIEiB7AAAADFBMVEX8AABHcEz6AAD6AADtPLgpAAAABHRSTlP+AGG+hRXDYwAABAxJREFUSMfNlzFv20YUx59IqACR0JvST3DpBwgY1EXRCugUFAUEWEeexStMSJNTIabdvSXgyQGSDsVl8tTAQzVlLRDEjA0vAYJOmjupQwsU7aChQZr0HcWj7o6xKS1FbxAk8f3u3nv3f++OQNce8B8j8WxtpBesjXTbayMZWRuBTpOtsJAIRAMRBxbCXPUtnBSjNgPzLKTXqh5BMWp+9n0LSf0mJHQsJKtyzN7K8ee0FkxuIbmV47SOZFMDCaGzayL1BJZ/KaQP4rPq2SdSDHXkS9MxzPFyV906kkM5gsqu54RLxFkJSf0+0AMtmxltQrpt7qoshq06MqEuKiL7bXJSITlJWrSUWYEc2sGfKW+h0nHXi8ofof8u5FAtDZWOs7ZS5rsRaX6uIRg7EO5oyJmNSKfONIRhjjtJqczIK5Gv35bjr4WCFsIEJX3c/q6nIfJpqjIr85F0aNTSkMTHQPJABVbaGQgntO9pSOrzVqhKpL+w802EBZS1dcRj7b6q/qjw/7VP707K8aOcyKM8MJBQcNfIUORbRdmiCdFjwWiTlol4VpYd2p3qGUOnMu9qJBO5Xi9MKjS4GklvOEaJwUZfJawMuob0PvQNJIVjlbBFabh9G2HgmYUM4NCrkRCI1WFAmZwW2/KmEaEJWOcLa9cc821kowFJry2iBa0fmP2RB7Zfs8wzw78Om4ZJYndch45hZiT519zRDAYD6yCMccYQXFEh+AOj0TxLLD8HubROwa2a0hjniHXPOJgHYVY8xK0gJRLm0kD3jFkJTF2xmFqUyBgN7opEmzh6ZfZkXt49TstYcBESwpTB5Uf/wDr3x6gvDiKElS4YEvlK9vPM0Y/YBuQpyhbT5clDZjWEF4fGE4wmsBrGpUi6WMQV3GPupTEbSBdQOU/Qr2XrV2N4CTKGv3ERmS9iO3bvistVjtWDG5k4qyZZZmAWolwyf2UkRc2NwVNbufPsWfgmnMXiHp3Pf9qbx3tSLfhx+uj3X6bl7m9IzXSUYHq3bvHX0wsuhvTCu3j4nH0nq/Hnl9HHN91PlZIFLiILZiHF3vHxKOocbWHG7m/eDoaMHKEOgwMuXtwOlud+jr4p8W/tcBqTB0OJ/HPzo8dM3EFpd7a36H2yRGSJxqqTIRLnZH8fkc//+ADcAulP6Yg+JGS5SjZHv2YVMpqQ4ReIfPvD+4QyWiADC9lG39RGIrIfka0jRNLvb5BBgUSdkXRMQ5ZdukAOGOFzRJLj6eYIke0i/EfvmUhatRREknPCiES4OHzJ6R1U27lMsoFEVefHHjaIX51EBJW8E4ud2YDu7i62kp6cLG9KqOVN61o8bbjzR2Bf4DPRgPTAluQ3TW8WKaz6krS8KG6s/S62N1sb+d+9V/4Lb2fNIm6fVJUAAAAASUVORK5CYII=",
                }
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct NavTitle {
    title: String,
    id: u32,
    index: u32,
}
impl Display for NavTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 根据格式化选项调整显示
        if f.alternate() {
            // 美化格式 {:#}
            write!(f, "ID: {}\nTitle: \"{}\"", self.id, self.title)
        } else {
            // 普通格式 {}
            write!(f, "{}: {}", self.id, self.title)
        }
    }
}

pub fn Nav() -> Element {
    // 使用 use_resource 管理异步请求
    let data = use_resource(|| async {
        match reqwest::get(format!("{}api/title", crate::GLOBAL_URL)).await {
            Ok(res) => {
                // 打印响应状态用于调试
                println!("Response status: {}", res.status());
                let res = res
                    .json::<crate::models::PageResponseCommon<Vec<NavTitle>>>()
                    .await
                    .map_err(|e| e.to_string());
                match res {
                    Ok(data) => Ok(data.data.items),
                    Err(_) => Ok(vec![]),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    });

    // 渲染逻辑
    match data.read_unchecked().as_ref() {
        Some(Ok(items)) => {
            // 默认选中第一个标签（如果存在）
            let default_id = items.first().map(|item| item.id).unwrap_or(0);

            // 转换 `NavTitle` 为 `TabTitle`（避免存储 `tabs_signal`，因为 `items` 已经是响应式的）
            let tabs: Vec<myw::TabTitle> = items
                .iter()
                .map(|item| myw::TabTitle {
                    title: item.title.clone(),
                    id: item.id,
                })
                .collect();

            // 使用 `use_signal` 存储当前选中的 `tab_id`
            let mut selected_id = use_signal(|| default_id);

            // 定义回调：更新 `selected_id`，并可以在这里添加额外逻辑（如 API 请求）
            let on_tab_change = move |tab_id: u32| {
                println!("Tab changed to: {}", tab_id);
                selected_id.set(tab_id);
            };
            let id = use_signal(|| if tabs.len() > 0 { tabs[0].id } else { 1 });
            let tabs = use_signal(|| tabs);
            rsx! {
                div { style: "width: 100%; max-width: 1200px; margin: auto",
                    // 渲染标签页
                    myw::Tabs { tabs, active_id: id, on_tab_change }
                    Link { id: selected_id }

                }
            }
        }
        Some(Err(err)) => rsx! {
            div { "Error: {err}" }
        },
        None => rsx! {
            div { "Loading..." }
        },
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NavLink {
    id: u32,
    index: u32,
    title_id: u32,
    src: String,
    title: String,
}
impl Display for NavLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 根据格式化选项调整显示
        if f.alternate() {
            // 美化格式 {:#}
            write!(f, "ID: {}\nTitle: \"{}\"", self.id, self.title)
        } else {
            // 普通格式 {}
            write!(f, "{}: {}", self.id, self.title)
        }
    }
}

#[component]
pub fn Link(id: Signal<u32>) -> Element {
    let mut future = use_resource(move || async move {
        let url = format!("{}api/link/{}", crate::GLOBAL_URL, id);
        console::log_1(&"this si run ".into());
        match reqwest::get(&url).await {
            Ok(res) => {
                let res = res
                    .json::<crate::models::PageResponseCommon<Vec<NavLink>>>()
                    .await
                    .map_err(|e| e.to_string());

                match res {
                    Ok(data) => {
                        console::log_1(&"ok".into());
                        Ok(data.data.items)
                    }
                    Err(_) => {
                        console::log_1(&"error".into());
                        Ok(vec![])
                    }
                }
            }
            Err(err) => Err(err.to_string()),
        }
    });

    use_effect(move || future.restart());

    match &*future.read_unchecked() {
        Some(Ok(items)) => rsx! {
            div {
                class: "link-list",
                style: "  display: grid;
            gap: 4px 4px;
            grid-template-columns: repeat(auto-fill, minmax(125px, 1fr));
            padding: 4px 0px;
            max-width: 1200px;
            margin: auto",

                // ✅ 正确方式：直接在 rsx! 内联迭代器
                {
                    items
                        .iter()
                        .map(|link| {
                            let src = link.src.clone();
                            rsx! {
                                myw::Button { key: "{link.id}", onclick: move |_| myw::util::window_open(&src), "{link.title}" }
                            }
                        })
                }
            }
        },
        Some(Err(e)) => rsx! {
            div { "Error: {e}" }
        },
        None => rsx! {
            div { "Loading..." }
        },
    }
}

#[component]
pub fn Search() -> Element {
    let mut text = use_signal(|| String::new());
    let search_click = move || {
        let search_term = text.read().trim().to_string();
        myw::util::window_open(&format!("https://www.baidu.com/s?wd={}", search_term));
    };
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == Key::Enter {
            search_click();
        }
    };
    rsx! {
        div { style: "width: 100%; text-align: center;",
            div { style: "margin: auto; max-width: 600px; width: 100%; position: relative;",
                //   position: relative;
                input {
                    r#type: "text",
                    value: text,
                    class: "myw-input",
                    style: " height: 40px;max-width: 600px; width: 100%; padding-right: 60px;",
                    placeholder: "请输入搜索的内容，键入Enter调用百度搜索",
                    oninput: move |e| text.set(e.value()),
                    onkeydown: handle_keydown,
                }
                myw::Button {
                    style: "position: absolute; right:2px; top: 1px; width: 50px",
                    border: "none",
                    onclick: search_click,
                    myw::icon::Search {}
                }
            }
        }
    }
}
#[component]
pub fn Beian() -> Element {
    // Base64 encoded image data for the badge icon
    const BADGE_ICON: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAMAAAC6V+0/AAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAACHUExURUdwTOrNevfowO/CcfLiqenQk5uNe+K0WeCvYvnvwvrkjJhxWd2kVdyxfsqEW8N/PtGudurCXAICcNVAJNqURskGCf3LVu/NafrdbNEgFe+3TOimRt9kL9Z9N3N9icy9fEcRQPB+M2VcacJWKaulfLiNU45gOg4lkFYuUOSVOI4II/0sD6WFVa28QBwAAAARdFJOUwDsXcJAj+/+5iG1/MquXZxvQ0cHGAAAAQlJREFUGNNN0NlywjAMBVCyOiFQalte4t1ZYeD/v6920zLoQQ9ndD2yTqej+rJty/70WfV12LbtNVzLD1zDNpuIlrC+qa/uQ2SUMr3cz/+GwhTHcaQj4y4Ux8N1S16GJaLMeNL2R7glK2UjzeyJO2f91gtZI5sBwxjTJG8SNg+/T3ymxhqQE/EqY2fFTuZIqaHGkF0+ul9UjrCnmW18MuK4zXgF7vfAWNqJBuIlyvEKkHSEkHBPzUmkq4QFwEP4wS1omhzWoIuEJdJgueRcp2YB4TpvX2JsQWCEhQAQRX18s8NJcmkkcHfgWQqEEivQaVr+3alBiiulNMZKFM37ol+3y6XrLlVzZH8APkEYkQTXpHkAAAAASUVORK5CYII=";

    rsx! {
        div { style: "text-align: center;",
            div { style: "text-align: center; margin-top: 40px",
                a {
                    href: "http://mayiwen.com/",
                    target: "_blank",
                    style: "color:#666; text-decoration: none",
                    "mayiwen.com"
                }
            }
            div { style: "text-align: center; margin-top: -0px",
                a {
                    href: "https://beian.miit.gov.cn/",
                    target: "_blank",
                    style: "color:#666",
                    "豫ICP备2022018473号-1"
                }
            }
            div { style: "text-align: center; margin-top: -3px",
                a {
                    target: "_blank",
                    href: "http://www.beian.gov.cn/portal/registerSystemInfo?recordcode=41162402000184",
                    style: "position: relative",

                    img {
                        src: BADGE_ICON,
                        style: "position: absolute; left: -20px; top: 0px",
                    }

                    p { style: "height:20px;line-height:20px; color:#666; display: inline-block; margin: 0; text-decoration: underline",
                        "豫公网安备 41162402000184号"
                    }
                }
            }
            // Spacer
            div { style: "height: 60px" }
        }
    }
}
// 定义函数
fn handle_button_click(link: &NavLink) {
    myw::util::window_open(&link.src);
}
