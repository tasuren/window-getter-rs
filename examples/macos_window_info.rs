fn main() {
    #[cfg(not(target_os = "macos"))]
    compile_error!("This example only supports macOS.");

    for window in window_getter::get_windows().unwrap() {
        let window_info = window.into_inner().into_window_info();

        println!(
            "{:?} ({}) memory usage: {} bytes",
            window_info.name().map(|name| name.to_string()),
            window_info.number().as_i64().unwrap(),
            window_info.memory_usage().as_i64().unwrap()
        );
    }
}
