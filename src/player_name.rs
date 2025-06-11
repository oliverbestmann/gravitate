#[cfg(target_arch = "wasm32")]
fn lookup() -> Option<String> {
    let Some(window) = web_sys::window() else {
        return "Unknown".into();
    };

    window
        .get("Player")
        .and_then(|f| f.as_string())
        .filter(|name| name.chars().any(|ch| !ch.is_whitespace()))
}

#[cfg(not(target_arch = "wasm32"))]
fn lookup() -> Option<String> {
    std::env::var("USER")
        .ok()
        .filter(|name| name.chars().any(|ch| !ch.is_whitespace()))
}
