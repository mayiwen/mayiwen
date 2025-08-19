use std::fmt::Display;

use super::backend;
use super::models;
use crate::myw::{self, Tab, TabColumn};
use crate::GLOBAL_USER_AUTH;
use dioxus::events::KeyboardEvent;
use dioxus::html::h1;
use dioxus::{html::view, prelude::*};
use keyboard_types::Key;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::{self, Deserialize, Serialize};
use web_sys;
use web_sys::console;
// 此层主要是为了发送后端接口。

pub async fn fetch_nav_titles() -> Result<Vec<models::NavTitle>, String> {
    match reqwest::get(format!("{}api/title", crate::GLOBAL_URL)).await {
        Ok(res) => {
            println!("Response status: {}", res.status());
            let res = res
                .json::<crate::models::PageResponseCommon<Vec<models::NavTitle>>>()
                .await
                .map_err(|e| e.to_string());
            match res {
                Ok(data) => Ok(data.data.items),
                Err(_) => Ok(vec![]),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
pub async fn fetch_nav_link(id: u32) -> Result<Vec<models::NavLink>, String> {
    match reqwest::get(format!("{}api/link/{}", crate::GLOBAL_URL, id)).await {
        Ok(res) => {
            println!("Response status: {}", res.status());
            let res = res
                .json::<crate::models::PageResponseCommon<Vec<models::NavLink>>>()
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
}

// pub async fn fetch_login(login_params: &models::Login) -> Result<models::LoginResult, String> {
//     let client = Client::new();
//     match client
//         .request(
//             reqwest::Method::OPTIONS,

//             "https://mayiwen.com:3000/auth/login",
//         )
//         .json(&login_params)
//         .header("Content-Type", "application/json")
//         .send()
//         .await
//     {
//         Ok(res) => {
//             println!("Response status: {}", res.status());
//             res.json::<models::LoginResult>()
//                 .await
//                 .map_err(|e| e.to_string())
//         }
//         Err(err) => Err(err.to_string()),
//     }
// }
pub async fn fetch_login(login_params: &models::Login) -> Result<models::LoginResult, String> {
    let client = Client::new();
    match client
        .post(format!("{}api/auth/login", crate::GLOBAL_URL))
        .json(&login_params)
        // .header("Content-Type", "application/json")
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            let res = res
                .json::<crate::models::ResponseCommon<models::LoginResult>>()
                .await
                .map_err(|e| e.to_string());
            match res {
                Ok(data) => {
                    console::log_1(&"ok".into());
                    Ok(data.data)
                }
                Err(_) => {
                    console::log_1(&"error".into());
                    Ok(models::LoginResult {
                        access_token: "错误的token".to_string(),
                    })
                }
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn fetch_title_add(
    title_add_params: &models::TitleAddParams,
) -> Result<models::NavTitleResult, String> {
    let client = Client::new();
    // 1. 创建自定义 HeaderMap
    let mut headers = HeaderMap::new();

    // 2. 手动插入 `Authorization`（保持大写）
    headers.insert(
        "Authorization", // 这里可以保持大写（但 reqwest 可能仍然会转换）
        HeaderValue::from_str(&format!("Bearer {}", GLOBAL_USER_AUTH.read().to_string())).unwrap(),
    );
    match client
        .post(format!("{}api/title", crate::GLOBAL_URL))
        .json(&title_add_params)
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            let res = res
                .json::<crate::models::ResponseCommon<models::NavTitleResult>>()
                .await
                .map_err(|e| e.to_string());
            match res {
                Ok(data) => Ok(data.data),
                Err(_) => Err("错误".to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
pub async fn fetch_title_del(id: u32) -> Result<crate::models::ResponseCommonSimple, String> {
    let client = Client::new();
    // 1. 创建自定义 HeaderMap
    let mut headers = HeaderMap::new();

    // 2. 手动插入 `Authorization`（保持大写）
    headers.insert(
        "Authorization", // 这里可以保持大写（但 reqwest 可能仍然会转换）
        HeaderValue::from_str(&format!("Bearer {}", GLOBAL_USER_AUTH.read().to_string())).unwrap(),
    );

    match client
        .delete(format!("{}api/title/{}", crate::GLOBAL_URL, id))
        // .json()
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            res.json::<crate::models::ResponseCommonSimple>()
                .await
                .map_err(|e| e.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn fetch_title_update(
    update_add_params: &models::NavTitle,
) -> Result<crate::models::ResponseCommon<models::NavTitleResult>, String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    // 2. 手动插入 `Authorization`（保持大写）
    headers.insert(
        "Authorization", // 这里可以保持大写（但 reqwest 可能仍然会转换）
        HeaderValue::from_str(&format!("Bearer {}", GLOBAL_USER_AUTH.read().to_string())).unwrap(),
    );
    match client
        .put(format!(
            "{}api/title/{}",
            crate::GLOBAL_URL,
            update_add_params.id
        ))
        .json(update_add_params)
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            let res = res
                .json::<crate::models::ResponseCommon<models::NavTitleResult>>()
                .await
                .map_err(|e| e.to_string());
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("错误".to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
pub async fn fetch_link_add(params: &models::NavLinkAdd) -> Result<models::NavLink, String> {
    let client = Client::new();
    // 1. 创建自定义 HeaderMap
    let mut headers = HeaderMap::new();

    // 2. 手动插入 `Authorization`（保持大写）
    headers.insert(
        "Authorization", // 这里可以保持大写（但 reqwest 可能仍然会转换）
        HeaderValue::from_str(&format!("Bearer {}", GLOBAL_USER_AUTH.read().to_string())).unwrap(),
    );
    match client
        .post(format!("{}api/link", crate::GLOBAL_URL))
        .json(&params)
        // .header("Content-Type", "application/json")
        // .header("Authorization", format!("Bearer {}", tokin))
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            let res = res
                .json::<crate::models::ResponseCommon<models::NavLink>>()
                .await
                .map_err(|e| e.to_string());
            match res {
                Ok(data) => Ok(data.data),
                Err(_) => Err("错误".to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn fetch_link_update(
    params: &models::NavLinkAdd,
    id: u32,
) -> Result<crate::models::ResponseCommon<models::NavLink>, String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    // 2. 手动插入 `Authorization`（保持大写）
    headers.insert(
        "Authorization", // 这里可以保持大写（但 reqwest 可能仍然会转换）
        HeaderValue::from_str(&format!("Bearer {}", GLOBAL_USER_AUTH.read().to_string())).unwrap(),
    );
    match client
        .put(format!("{}api/link/{}", crate::GLOBAL_URL, id))
        .json(params)
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            let res = res
                .json::<crate::models::ResponseCommon<models::NavLink>>()
                .await
                .map_err(|e| e.to_string());
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("错误".to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn fetch_link_del(id: u32) -> Result<crate::models::ResponseCommonSimple, String> {
    let client = Client::new();
    // 1. 创建自定义 HeaderMap
    let mut headers = HeaderMap::new();

    // 2. 手动插入 `Authorization`（保持大写）
    headers.insert(
        "Authorization", // 这里可以保持大写（但 reqwest 可能仍然会转换）
        HeaderValue::from_str(&format!("Bearer {}", GLOBAL_USER_AUTH.read().to_string())).unwrap(),
    );

    match client
        .delete(format!("{}api/link/{}", crate::GLOBAL_URL, id))
        // .json()
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            res.json::<crate::models::ResponseCommonSimple>()
                .await
                .map_err(|e| e.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn fetch_nav_link_update_list(
    params: &Vec<models::NavLinkSort>,
) -> Result<crate::models::ResponseCommon<Vec<models::NavLink>>, String> {
    let client = Client::new();
    // 1. 创建自定义 HeaderMap
    let mut headers = HeaderMap::new();

    // 2. 手动插入 `Authorization`（保持大写）
    headers.insert(
        "Authorization", // 这里可以保持大写（但 reqwest 可能仍然会转换）
        HeaderValue::from_str(&format!("Bearer {}", GLOBAL_USER_AUTH.read().to_string())).unwrap(),
    );
    match client
        .post(format!("{}api/link/sort", crate::GLOBAL_URL))
        .json(&params)
        // .header("Content-Type", "application/json")
        // .header("Authorization", format!("Bearer {}", tokin))
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            res.json::<crate::models::ResponseCommon<Vec<models::NavLink>>>()
                .await
                .map_err(|e| e.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn fetch_nav_title_update_list(
    params: &Vec<models::NavTitleSort>,
) -> Result<crate::models::ResponseCommon<Vec<models::NavTitle>>, String> {
    let client = Client::new();
    // 1. 创建自定义 HeaderMap
    let mut headers = HeaderMap::new();

    // 2. 手动插入 `Authorization`（保持大写）
    headers.insert(
        "Authorization", // 这里可以保持大写（但 reqwest 可能仍然会转换）
        HeaderValue::from_str(&format!("Bearer {}", GLOBAL_USER_AUTH.read().to_string())).unwrap(),
    );
    match client
        .post(format!("{}api/title/sort", crate::GLOBAL_URL))
        .json(&params)
        // .header("Content-Type", "application/json")
        // .header("Authorization", format!("Bearer {}", tokin))
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => {
            println!("Response status: {}", res.status());
            res.json::<crate::models::ResponseCommon<Vec<models::NavTitle>>>()
                .await
                .map_err(|e| e.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}
