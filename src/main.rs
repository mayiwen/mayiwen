use dioxus::prelude::*;
mod about;
mod code;
mod home;
mod models;
mod myw;
mod nuoruo;
use web_sys::window;
static GLOBAL_USER_AUTH: GlobalSignal<String> = Signal::global(|| "".to_string());
static GLOBAL_URL: &'static str = "https://mayiwen.com:7523/";
// use myw;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/code")]
    Code {
    },
    #[route("/yueduqi")]
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
        document::Link { rel: "icon", r#type: "image/x-icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no",
        }
        // 可选：添加其他 meta 标签
        meta { name: "apple-mobile-web-app-capable", content: "yes" }
        meta { name: "format-detection", content: "telephone=no" }
        Router::<Route> {
        }
    }
}

#[component]
pub fn Code() -> Element {
    rsx! {
        code::Index {}
    }
}
#[component]
pub fn Home() -> Element {
    rsx! {
        home::Index {}
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
        about::Index {}
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
    let super_message_signal: Signal<Vec<myw::MessageItem>> = use_signal(|| vec![]);
    // super_message_signal.
    use_context_provider(|| super_message_signal);
    // myw::AppState::new();
    //   // 提供共享状态（与你的原代码完全兼容）
    provide_context(use_signal(|| {
        vec![
            // myw::MessageItem {
            //     t: myw::MessageType::INFO,
            //     m: "rust全栈构建网站技术展示".to_string(),
            //     pin: false,
            // },
            // myw::MessageItem {
            //     t: myw::MessageType::INFO,
            //     m: "rust全栈构建网站技术展示".to_string(),
            //     pin: false,
            // },
            myw::MessageItem {
                t: myw::MessageType::INFO,
                m: "欢迎访问 mayiwen.com".to_string(),
                pin: false,
            },
            myw::MessageItem {
                t: myw::MessageType::INFO,
                m: "点击左上角“阅读”按钮，前往一文小说阅读器主页".to_string(),
                pin: false,
            },
        ]
    }));

    rsx! {
        div { id: "navbar", style: "padding: 4px",
            div { style: "position: absolute; right:3px; top: 3px",
                myw::Button {
                    border: "none",
                    onclick: move || {
                        if let Some(window) = window() {
                            count.set(count() + 1);
                            let document = window.document().unwrap();
                            let html_element = document.document_element().unwrap();
                            html_element
                                .set_attribute(
                                    "data-mayiwen-theme",
                                    if count() % 2 == 1 { "black" } else { "none" },
                                )
                                .unwrap_or_else(|e| log::error!("Failed to set attribute: {:?}", e));
                        }
                    },
                    myw::icon::Theme {}
                }
            }
            Link { to: Route::Home {},
                myw::Button {
                    border: if (route == Route::Home {}) { "both" } else { "none" },
                    active: route == Route::Home {},
                    style: "vertical-align: middle; line-heght: 36px!important;",
                    myw::icon::Myw {}
                    span { style: " margin-top: 7px;display: inline-block;", {"马一文"} }
                }
            }
            Link { to: Route::Code {},
                myw::Button {
                    border: if matches!(route, Route::Code { .. }) { "both" } else { "none" },
                    active: matches!(route, Route::Code { .. }),
                    style: "vertical-align: top;",
                    {"☰ 代码"}
                }
            }
            Link { to: Route::Nuoruo {},
                myw::Button {
                    border: if matches!(route, Route::Nuoruo { .. }) { "both" } else { "none" },
                    active: matches!(route, Route::Nuoruo { .. }),
                    style: "vertical-align: top;",
                    {"✤ 阅读"}
                }
            }
            Link { to: Route::About {},
                myw::Button {
                    border: if matches!(route, Route::About { .. }) { "both" } else { "none" },
                    active: matches!(route, Route::About { .. }),
                    style: "vertical-align: top;",
                    {"❃ 设置"}
                }
            }
                // "当前路径: ",
        // {router.current_path()}
        //  "当前路由: {route:?}"  // 这会显示匹配的路由枚举变体

        }
        div {
            style: "height: calc(100% - 46px); overflow: auto",
            Outlet::<Route> {}
            home::core::Beian {}
            myw::AddMessage {}
        }

    }
}
