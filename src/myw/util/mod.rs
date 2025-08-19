use web_sys;

pub fn window_open(url: &str) {
    if let Some(win) = web_sys::window() {
        let _ = win.open_with_url(&url);
    }
}
