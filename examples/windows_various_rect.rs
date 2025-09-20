#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("This example only supports windows.");
}

#[cfg(target_os = "windows")]
fn main() {
    for window in window_getter::get_windows().unwrap() {
        let window = window.into_platform_window();

        println!("\n{:?} ({:?})", window.title(), window.hwnd());
        println!("\tGetWindowRect: {:?}", window.rect());
        println!(
            "\tDwmGetWindowAttribute with DWMWA_EXTENDED_FRAME_BOUNDS: {:?}",
            window.extended_frame_bounds()
        );
    }
}
