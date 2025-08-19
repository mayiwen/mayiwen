use dioxus::prelude::*;
use dioxus_router::prelude::*;
mod about;
mod code;
mod home;
mod models;
mod myw;
mod nuoruo;
use gloo_timers::callback::Interval;
use std::collections::VecDeque;
use web_sys::window;
static GLOBAL_USER_AUTH: GlobalSignal<String> = Signal::global(|| "".to_string());
static GLOBAL_URL: &'static str = "https://mayiwen.com:7523/";
// use myw;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/code")]
    Code {
    },
    #[route("/nuoruo")]
    Nuoruo {},
    #[route("/about")]
    About {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 直接定义全局 Signal（无需结构体）

    rsx! {
        document::Link { rel: "icon",
            type: "image/x-icon",
            href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {
        }
    }
}

#[component]
pub fn Code() -> Element {
    rsx! {
        code::Index {  }
    }
}
#[component]
pub fn Home() -> Element {
    rsx! {
        home::Index {  }
    }
}
#[component]
pub fn Nuoruo() -> Element {
    rsx! {
       nuoruo::Index {}
    }
}
#[component]
pub fn About() -> Element {
    rsx! {
        about::Index {  }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    let route = use_route::<Route>(); // 替代 use_router()

    // 获取当前完整路径（如 "/user/123"）
    // let current_path = router.current_path();
    // use_context_provider(|| AppState {
    //     items: vec!["Item 1".into(), "Item 2".into()],
    // });
    //
    let mut count: Signal<u32> = use_signal(|| 0);
    // 定义一个全局的变量，给所有的组件使用。
    //
    let mut super_message_signal: Signal<Vec<myw::MessageItem>> = use_signal(|| vec![]);
    // super_message_signal.
    use_context_provider(|| super_message_signal);
    // myw::AppState::new();
    //   // 提供共享状态（与你的原代码完全兼容）
    provide_context(use_signal(|| {
        vec![
            myw::MessageItem {
                t: myw::MessageType::INFO,
                m: "rust全栈构建网站技术展示".to_string(),
                pin: false,
            },
            myw::MessageItem {
                t: myw::MessageType::INFO,
                m: "欢迎访问 mayiwen.com".to_string(),
                pin: false,
            },
        ]
    }));

    rsx! {
        div {
            id: "navbar",
            style: "padding: 4px",
            div {
                style: "position: absolute; right:3px; top: 3px",
                myw::Button{
                    border: "none",
                    onclick:move || {
                        if let Some(window) = window() {
                            count.set(count() + 1);
                                              let document = window.document().unwrap();
                                              let html_element = document.document_element().unwrap();
                                              html_element.set_attribute(
                                                  "data-mayiwen-theme",
                                                  if count() % 2 == 1 { "black" } else { "none" } // 直接解引用 count
                                              ).unwrap_or_else(|e| log::error!("Failed to set attribute: {:?}", e));
                                          }
                    },
                    myw::icon::Theme{}}
            },
            Link {
                to: Route::Home {},
                myw::Button {
                    border: if (route == Route::Home{}) {"both"} else {"none"},
                    active: route == Route::Home{},
                    style: "vertical-align: top;",
                    myw::icon::Myw{}
                    {"马一文"}
                },
            }
            Link {
                to: Route::Code {},
                 myw::Button {
                    border: if  matches!(route, Route::Code { .. }) {"both"} else {"none"},
                    active:  matches!(route, Route::Code { .. }),
                    style: "vertical-align: top;",
                    {"☰ 代码"}
                },
            },
            Link {
                to: Route::Nuoruo  {},
                 myw::Button {
                    border: if  matches!(route, Route::Nuoruo { .. }) {"both"} else {"none"},
                    active:  matches!(route, Route::Nuoruo { .. }),
                    style: "vertical-align: top;",
                    {"✤ 诺若"}
                },
            },
            Link {
                to: Route::About {},
                 myw::Button {
                    border: if  matches!(route, Route::About { .. }) {"both"} else {"none"},
                    active:  matches!(route, Route::About { .. }),
                    style: "vertical-align: top;",
                    {"❃ 设置"}
                },
            },
            // "当前路径: ",
            // {router.current_path()}
            //  "当前路由: {route:?}"  // 这会显示匹配的路由枚举变体

        }

        Outlet::<Route> {}
        home::core::Beian{}
        myw::AddMessage{}
    }
}
