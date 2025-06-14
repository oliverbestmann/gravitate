pub fn _lookup() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    let name = {
        let Some(window) = web_sys::window() else {
            return None;
        };

        window.get("Player").and_then(|f| f.as_string())
    };

    #[cfg(not(target_arch = "wasm32"))]
    let name = std::env::var("USER").ok();

    name.filter(|name| name.chars().any(|ch| !ch.is_whitespace()))
        .map(|name| name.trim().to_owned())
}
