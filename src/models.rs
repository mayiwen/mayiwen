use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
struct AuthState {
    token: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageResponseCommon<T> {
    pub code: u64,
    pub message: String,
    pub data: PageResponse<T>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageResponse<T> {
    pub page: u64,
    pub size: u64,
    pub total: u64,
    pub items: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCommon<T> {
    pub code: u64,
    pub message: String,
    pub data: T,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCommonSimple {
    pub code: u64,
    pub message: String,
}
