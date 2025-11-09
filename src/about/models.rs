
use dioxus::prelude::*;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NavTitle {
    pub title: String,
    pub id: u32,
    pub index: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NavTitleResult {
    pub title: String,
    pub id: u64,
    pub index: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Login {
    pub name: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LoginResult {
    pub access_token: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct NavTableProps {
    pub items: Signal<Vec<NavTitle>>,
    pub add_result: EventHandler<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TitleAddParams {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TitleAddResult {
    pub id: u32,
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NestjsResponse<T> {
    pub raw: Vec<T>,
    pub affected: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NavLink {
    pub title: String,
    pub id: u32,
    pub src: String,
    pub title_id: u32,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NavLinkAdd {
    pub index: u32,
    pub title_id: u32,
    pub src: String,
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NavLinkSort {
    pub index: u32,
    pub id: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NavTitleSort {
    pub index: u32,
    pub id: u32,
}
