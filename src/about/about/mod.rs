use std::fmt::Display;

use crate::myw::{self, Tab};
use dioxus::events::KeyboardEvent;
use dioxus::html::h1;
use dioxus::{html::view, prelude::*};
use keyboard_types::Key;
use serde::{self, Deserialize, Serialize};
use web_sys;
#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            myw::Gap {  }
            h2 {
                "马一文的个人网站"
            }

            p {"本站为非商业性质开源网站，仅用于个人学习网站建设技术。"}
            myw::Gap {  }
            h2 {"鸣谢。本站使用到的开源技术："}
            p {"基础: html、css、rust"}
            p{"前端框架：dioxus、自制组件"}
            p{"后端框架：axum"}
            p{"orm：seaorm"}
            p{"数据库：postgresSql"}
            p{"服务器：nginx"}
            p{"跨平台客户端：tauri"}
            p{"云服务器、域名：腾迅云"}
            p{"代码平台：https://gitlink.org.cn/mayiwen"}
            p{"js、scss、remixicon、web_sys、reqwest、serde、tokio、gloo_timers、"}


            myw::Gap {  }
            h2 {
                "联系我"
            }
            p {"建议与反馈：i@mayiwen.com"}
                  myw::Gap {  }
            h2 {
                "版本"
            }
            p {"1.2.2"}
            p {"更新日期：2025-08-19"}
            myw::Gap {  }
            h2 {
                "重要版本记录"
            }

            p {"2025-08-10 1.2.1版本从js切到rust技术栈"}

        }
    }
}
