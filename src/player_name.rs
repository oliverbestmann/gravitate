#[cfg(target_arch = "wasm32")]
fn _lookup() -> Option<String> {
    let Some(window) = web_sys::window() else {
        return "Unknown".into();
    };

    window
        .get("Player")
        .and_then(|f| f.as_string())
        .filter(|name| name.chars().any(|ch| !ch.is_whitespace()))
        .map(String::from)
}

#[cfg(not(target_arch = "wasm32"))]
fn _lookup() -> Option<String> {
    std::env::var("USER")
        .ok()
        .filter(|name| name.chars().any(|ch| !ch.is_whitespace()))
        .map(String::from)
}
