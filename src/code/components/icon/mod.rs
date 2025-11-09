
use crate::myw::{self};
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    rsx! {
        myw::Gap { }
        myw::icon::Myw{},
        myw::icon::Close{}
    }
}
