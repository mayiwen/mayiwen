use dioxus::signals::Readable;

use super::backend;
use super::models;
use crate::GLOBAL_USER_AUTH;
// 此层主要为了调用接口。是视图与后端的中间层。 给出参数，得到结果。

pub async fn fetch_nav_titles() -> Vec<models::NavTitle> {
    let mut vec: Vec<models::NavTitle> = vec![];
    let res = backend::fetch_nav_titles().await;
    match res {
        Ok(ret) => vec = ret,
        Err(_) => {}
    }
    vec
}
pub async fn fetch_nav_link(id: u32) -> Vec<models::NavLink> {
    let mut vec: Vec<models::NavLink> = vec![];
    let res = backend::fetch_nav_link(id).await;
    match res {
        Ok(ret) => vec = ret,
        Err(_) => {}
    }
    vec
}

pub async fn fetch_login(login_params: &models::Login) -> Result<models::LoginResult, String> {
    backend::fetch_login(login_params).await
}

pub async fn fetch_title_add(
    title_add_params: &models::TitleAddParams,
) -> Result<models::NavTitleResult, String> {
    backend::fetch_title_add(&title_add_params).await
}

pub async fn fetch_title_del(id: u32) -> bool {
    let res = backend::fetch_title_del(id).await;
    let ret = match res {
        Ok(ret) => {
            if ret.code == 0 {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };
    ret
}

pub async fn fetch_title_update(update_params: &models::NavTitle) -> bool {
    let res = backend::fetch_title_update(update_params).await;
    let ret = match res {
        Ok(ret) => {
            if ret.code == 0 {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };
    ret
}

pub async fn fetch_link_add(params: &models::NavLinkAdd) -> Result<models::NavLink, String> {
    backend::fetch_link_add(&params).await
}

pub async fn fetch_link_update(params: &models::NavLinkAdd, id: u32) -> bool {
    let res = backend::fetch_link_update(params, id).await;
    let ret = match res {
        Ok(ret) => {
            if ret.code == 0 {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };
    ret
}
pub async fn fetch_link_del(id: u32) -> bool {
    let res = backend::fetch_link_del(id).await;
    let ret = match res {
        Ok(ret) => {
            if ret.code == 0 {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };
    ret
}

pub async fn fetch_nav_link_update_list(params: &Vec<models::NavLinkSort>) -> bool {
    let res = backend::fetch_nav_link_update_list(params).await;
    let ret = match res {
        Ok(_) => true,
        Err(_) => true,
    };
    ret
}

pub async fn fetch_nav_title_update_list(params: &Vec<models::NavTitleSort>) -> bool {
    let res = backend::fetch_nav_title_update_list(params).await;
    let ret = match res {
        Ok(_) => true,
        Err(_) => true,
    };
    ret
}
