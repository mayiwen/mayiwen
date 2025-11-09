use dioxus::prelude::*;
use gloo_timers::callback::Interval;
use std::rc::Rc;

pub mod icon;
pub mod util;
 // 轻量级定时器

// 定义组件的属性结构
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    // 必填属性
    // text: String,
    //
    //

    // 可选属性（带默认值）
    #[props(default = "both".to_string())]
    border: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "".to_string())]
    style: String,
    #[props(default = false)]
    active: bool,

    // 事件处理函数
    #[props(default)]
    onclick: EventHandler,

    // 子元素
    children: Element,
}
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // 判断边框的样式
    let border: String = if props.border == "both".to_string() {
        "1px solid var(--myw-border)".to_string()
    } else if props.border == "none".to_string() {
        "1px solid transparent".to_string()
    } else {
        "1px solid var(--myw-border)".to_string()
    };
    // 判断这个gbutton是否被选择中 选中的时候，加个active类。
    let is_active = if props.active {
        " active".to_string()
    } else {
        "".to_string()
    };
    // if props.tabset {
    //     border = "1px solid both!important".to_string();
    // } else {
    //     border = "1px solid transparent!important".to_string();
    // }

    rsx! {
        button {
            class: "myw-button {is_active} {props.class}",
            style: "border: {border}; {props.style}",
            onclick: move |_| props.onclick.call(()), // 必须显式绑定事件
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub title: String,
    pub content: Element, // 将 v 改为更明确的 content
}

#[component]
pub fn Tabset(tabs: Vec<Tab>) -> Element {
    let mut active_tab = use_signal(|| 0);

    rsx! {
        div { class: "tabset-container",

            // 标签栏
            div { class: "tab-header",
                for (index , tab) in tabs.iter().enumerate() {
                    Button {
                        key: "{index}",
                        active: if *active_tab.read() == index { true } else { false },
                        border: if *active_tab.read() == index { "both" } else { "none" },
                        onclick: move |_| active_tab.set(index),
                        "{tab.title}"
                    }
                }
            }
            div { style: "border-bottom: 1px solid var(--myw-border); text-align: left" }

            // 内容区
            div { class: "tab-content", {tabs.get(*active_tab.read()).map(|tab| tab.content.clone())} }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TabTitle {
    pub title: String,
    pub id: u32, // 将 v 改为更明确的 content
}

#[component]
pub fn Tabs(
    tabs: Signal<Vec<TabTitle>>, // 响应式标签数据
    active_id: Signal<u32>,
    // 新增：响应式活动标签ID
    on_tab_change: EventHandler<u32>, // 标签切换回调
) -> Element {
    // 确保默认选中第一个标签（如果active_id为0且存在标签）
    use_effect(move || {
        if *active_id.read() == 0 && !tabs.read().is_empty() {
            active_id.set(tabs.read()[0].id);
        }
    });

    rsx! {
        div {
            // 标签栏容器
            div { style: "border-bottom: 1px solid var(--myw-border); text-align: left",

                // 动态渲染标签按钮
                for tab in tabs.read().iter() {
                    {
                        let tab_id = tab.id;
                        rsx! {
                            Button {
                                key: "{tab_id}",
                                active: *active_id.read() == tab_id,
                                border: if *active_id.read() == tab_id { "both" } else { "none" },
                                onclick: move |_| {
                                    active_id.set(tab_id);
                                    on_tab_change.call(tab_id);
                                },
                                "{tab.title}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Gap(
    #[props(default = "")] w: &'static str,
    #[props(default = "")] h: &'static str,
    #[props(default = "")] width: &'static str,
    #[props(default = "")] height: &'static str,
) -> Element {
    // Determine which style to apply based on the props
    let style = if !w.is_empty() {
        format!("display: inline-block; width: {}px", w)
    } else if !h.is_empty() {
        format!("height: {}px", h)
    } else if !width.is_empty() {
        format!("display: inline-block; width: {}", width)
    } else if !height.is_empty() {
        format!("height: {}", height)
    } else {
        // Default case
        "height: 30px".to_string()
    };

    rsx! {
        div { style: "{style}" }
    }
}

#[component]
pub fn MessageView(message: MessageItem) -> Element {
    // 根据消息类型设置不同的样式
    let bg_color = match message.t {
        MessageType::WARNING => "var(--myw-yellowDefault)",
        MessageType::ERROR => "var(--myw-redDefault)",
        MessageType::INFO => "var(--myw-blueDefault)",
    };

    rsx! {
        p {
            style: format!(
                "background-color: {bg_color};
                                                                        border: 1px solid var(--myw-border);
                                                                        border-radius: 4px;
                                                                        padding: 4px;
                                                                        color: var(--myw-bc);
                                                                        margin-bottom: 4px;
                                                                        min-width: 300px;
                                                                        width: 100%;
                                                                        max-width: 800px;
                                                                        opacity: 0.9;",
            ),
            "{message.m}" // 显示实际消息内容
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum MessageType {
    WARNING,
    ERROR,
    INFO,
}
#[derive(Clone, PartialEq)]
pub struct MessageItem {
    /// type
    pub t: MessageType,
    /// message
    pub m: String,
    /// 是否固定
    pub pin: bool,
}

#[component]
pub fn AddMessage() -> Element {
    let messages = consume_context::<Signal<Vec<MessageItem>>>();
    let mut flag = false;
    // console::log_1(&format!("方法执行了").into());
    use_future(move || {
        let mut messages = messages.clone();
        async move {
            // 创建定时器
            let interval = Interval::new(1500, move || {
                // console::log_1(&"loop: run".into());
                messages.with_mut(|msgs| {
                    if flag {
                        if let Some(first) = msgs.first() {
                            // console::log_1(&format!("移除消息: {:?}", first).into());
                            msgs.remove(0);
                            flag = false;
                        }
                    } else {
                        if msgs.len() == 1 {
                            flag = true;
                        } else {
                            if let Some(first) = msgs.first() {
                                // console::log_1(&format!("移除消息: {:?}", first).into());
                                msgs.remove(0);
                            }
                        }
                    }
                });
            });

            // 保持 Future 存活
            std::future::pending::<()>().await;

            // 理论上不会执行到这里
            interval.cancel();
        }
    });

    rsx! {
        div { style: "position: fixed; left: 50%; top: 20px; transform: translate(-50%, 0); z-index: 100000000000000;",
            for message in messages.read().iter() {
                MessageView { message: message.clone() }
            }
        }
    }
}

#[derive(Clone)]
pub struct TabColumn<T>
where
    T: Clone + PartialEq + 'static,
{
    pub width: u32,
    pub title: &'static str,
    pub id: &'static str,
    pub renderer: Rc<dyn Fn(&T) -> Element>,
}

impl<T> PartialEq for TabColumn<T>
where
    T: Clone + PartialEq + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.title == other.title
            && self.id == other.id
            && Rc::ptr_eq(&self.renderer, &other.renderer)
    }
}

#[component]
pub fn Table<T: Clone + PartialEq + 'static>(
    columns: Signal<Vec<TabColumn<T>>>,
    data: Signal<Vec<T>>,
    row_height: Option<u32>,
    header_height: Option<u32>,
    key_fn: fn(&T) -> String,
    #[props(default)] on_row_index_change: EventHandler<(u32, u32)>, // 标签切换回调
) -> Element {
    let row_height = row_height.unwrap_or(40);
    let header_height = header_height.unwrap_or(40);
    let mut dragged_index = use_signal(|| None::<u32>);
    // 使用 use_memo 计算总宽度，当 columns 变化时自动重新计算
    let total_width =
        use_memo(move || columns.read().iter().map(|col| col.width + 9).sum::<u32>() + 1);

    rsx! {
        div { class: "table-container", style: "overflow: auto;",

            div { style: "width: {total_width}px;",

                // Header row - 当 columns 变化时会自动重新渲染
                div {
                    class: "table-header",
                    style: "font-size: 0; height: {header_height}px; margin-bottom: 1px;",

                    for (index , col) in columns.read().iter().enumerate() {
                        div {
                            class: "table-header-cell ellipsis",
                            style: format!(
                                "display: inline-block; height: {header_height}px; line-height: {header_height}px; width: {}px; font-size: 16px; padding: 0 4px; border: 1px solid var(--myw-border);border-left: {};background-color: var(--myw-boxBc);",
                                col.width,
                                if index == 0 { "1px solid var(--myw-border)" } else { "none" },
                            ),
                            key: "{col.id}",
                            title: "{col.title}",
                            "{col.title}"
                        }
                    }
                }

                // Table body - 当 data 变化时会自动重新渲染
                div { class: "table-body",

                    for (row_index , item) in data.read().iter().enumerate() {
                        div {
                            class: "table-row",
                            key: "{key_fn(item)}",
                            style: "font-size: 0;",
                            draggable: true,
                            ondragstart: move |_| {
                                dragged_index.set(Some(row_index as u32));
                            },
                            ondragover: move |e| {
                                e.prevent_default();
                            },
                            ondrop: move |_| {
                                if let Some(dragged_idx) = *dragged_index.read() {
                                    on_row_index_change.call((dragged_idx, row_index as u32));
                                }
                            },
                            ondragend: move |_| {
                                dragged_index.set(None);
                            },

                            // draggable="true" dragstart:="start" dragover:="over($)" drop:="drop()"
                            for (col_index , col) in columns.read().iter().enumerate() {
                                div {
                                    class: "table-cell ellipsis",
                                    style: format!(
                                        "display: inline-block;
                                                                        height: {row_height}px;
                                                                        line-height: {row_height}px;
                                                                        width: {}px;
                                                                        font-size: 16px;
                                                                        padding: 0 4px;
                                                                        border-right: 1px solid var(--myw-border);
                                                                        border-bottom: 1px solid var(--myw-border);
                                                                        background-color: var(--myw-bc);
                                                                        border-left: {};",
                                        col.width,
                                        if col_index == 0 { "1px solid var(--myw-border)" } else { "none" },
                                    ),
                                    key: "{col.id}-{key_fn(item)}",


                                    // 当数据变化时，闭包会自动获取最新值
                                    {(col.renderer)(item)}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Modal(
    is_open: Signal<bool>,
    title: Option<String>,
    children: Element,
    // 事件处理函数
    #[props(default)] on_confirm: EventHandler<bool>, // 新增确认回调
) -> Element {
    rsx! {
        // 背景遮罩层
        div {
            style: "position: fixed; height: 100dvh; width: 100%; background-color: rgba(128, 128, 128, 0.5); z-index: 100000; left: 0;top: 0; opcity: 0.5; ",
            hidden: !is_open(),
            onclick: move |_| is_open.set(false),
        }

        // Modal 内容
        dialog {
            open: is_open(),
            style: "border: 1px solid var(--myw-border);
                   background-color: var(--myw-bc);
                   border-radius: 4px;

                   opacity: 1;
                   position: fixed;
                   left: 50%;
                   top: 50%;
                   transform: translate(-50%, -50%);
                   z-index: 1000000;
                    background-color: var(--myw-bc);
                     color: var(--myw-color);",
            // 标题区域
            if let Some(title) = title {
                h3 { style: "padding-left: 4px",
                    "{title}"
                    Button {
                        onclick: move |_| is_open.set(false),
                        style: "float: right",
                        border: "none",
                        icon::Close {}
                    }
                }
            }
            div { style: "padding: 4px",
                // 内容区域
                {children}
            }


            div { style: "float: right",
                Button {
                    onclick: move |_| {
                        on_confirm.call(true);
                    },
                    "确认"
                }

                Gap { w: "10" }
                // 关闭按钮

                Button { border: "none", onclick: move |_| is_open.set(false), "取消" }
            }

        }
    }
}
