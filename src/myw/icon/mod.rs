use dioxus::prelude::*;
use web_sys::window;
#[component]
pub fn Loading(
    /// 图标宽度和高度（像素）
    #[props(default = 30)]
    wh: usize,
    /// 是否启用旋转动画
    #[props(default = false)]
    rotate: bool,
) -> Element {
    let size_str = format!("{}px", wh);
    let myw_rotate = if rotate { "myw-rotate" } else { "" };
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "var(--myw-color)",
            width: size_str.clone(),
            height:size_str,
            class: "{myw_rotate}",
            path {
                d: "M18.364 5.63604L16.9497 7.05025C15.683 5.7835 13.933 5 12 5C8.13401 5 5 8.13401 5 12C5 15.866 8.13401 19 12 19C15.866 19 19 15.866 19 12H21C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C14.4853 3 16.7353 4.00736 18.364 5.63604Z"
            }
        }
    }
}
#[component]
pub fn Search(
    /// 图标宽度和高度（像素）
    #[props(default = 24)]
    wh: usize,
    #[props(default = "".to_string())] style: String,
) -> Element {
    let size_str = format!("{}px", wh);
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "var(--myw-color)",
            width: size_str.clone(),
            height:size_str,
            style: "vertical-align: middle; {style}",
            path {
                d: "M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z"
            }
        }
    }
}

#[component]
pub fn Close(
    /// 图标宽度和高度（像素）
    #[props(default = 30)]
    wh: usize,
    #[props(default = "".to_string())] style: String,
) -> Element {
    let size_str = format!("{}px", wh);
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "var(--myw-color)",
            width: size_str.clone(),
            height:size_str,
            style: "vertical-align: middle; {style}",
            path {
                d: "M11.9997 10.5865L16.9495 5.63672L18.3637 7.05093L13.4139 12.0007L18.3637 16.9504L16.9495 18.3646L11.9997 13.4149L7.04996 18.3646L5.63574 16.9504L10.5855 12.0007L5.63574 7.05093L7.04996 5.63672L11.9997 10.5865Z
                "
            }
        }

    }
}

#[component]
pub fn Theme(
    /// 图标宽度和高度（像素）
    #[props(default = 24)]
    wh: usize,
    #[props(default = "".to_string())] style: String,
) -> Element {
    let size_str = format!("{}px", wh);
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "var(--myw-color)",
            width: size_str.clone(),
            height:size_str,
            style: "vertical-align: middle; {style}",
            path {
                d: "M15.1986 9.94447C14.7649 9.5337 14.4859 8.98613 14.4085 8.39384L14.0056 5.31138L11.275 6.79724C10.7503 7.08274 10.1433 7.17888 9.55608 7.06948L6.49998 6.50015L7.06931 9.55625C7.17871 10.1435 7.08257 10.7505 6.79707 11.2751L5.31121 14.0057L8.39367 14.4086C8.98596 14.4861 9.53353 14.7651 9.94431 15.1987L12.0821 17.4557L13.4178 14.6486C13.6745 14.1092 14.109 13.6747 14.6484 13.418L17.4555 12.0823L15.1986 9.94447ZM15.2238 15.5079L13.0111 20.1581C12.8687 20.4573 12.5107 20.5844 12.2115 20.442C12.1448 20.4103 12.0845 20.3665 12.0337 20.3129L8.49229 16.5741C8.39749 16.474 8.27113 16.4096 8.13445 16.3918L3.02816 15.7243C2.69958 15.6814 2.46804 15.3802 2.51099 15.0516C2.52056 14.9784 2.54359 14.9075 2.5789 14.8426L5.04031 10.3192C5.1062 10.1981 5.12839 10.058 5.10314 9.92253L4.16 4.85991C4.09931 4.53414 4.3142 4.22086 4.63997 4.16017C4.7126 4.14664 4.78711 4.14664 4.85974 4.16017L9.92237 5.10331C10.0579 5.12855 10.198 5.10637 10.319 5.04048L14.8424 2.57907C15.1335 2.42068 15.4979 2.52825 15.6562 2.81931C15.6916 2.88421 15.7146 2.95507 15.7241 3.02833L16.3916 8.13462C16.4095 8.2713 16.4739 8.39766 16.5739 8.49245L20.3127 12.0338C20.5533 12.2617 20.5636 12.6415 20.3357 12.8821C20.2849 12.9357 20.2246 12.9795 20.1579 13.0112L15.5078 15.224C15.3833 15.2832 15.283 15.3835 15.2238 15.5079ZM16.0206 17.435L17.4348 16.0208L21.6775 20.2634L20.2633 21.6776L16.0206 17.435Z "
            }
        }

    }
}

#[component]
pub fn Myw() -> Element {
    rsx! {
        img {
            height: "30px",
            width: "30px",
            style: "vertical-align: top;",
            src: "data:image/x-icon;base64,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"
         }
    }
}
