use window_getter::platform_impl::macos::permission;

fn main() {
    let screen_capture_access = permission::has_screen_capture_access();
    println!("screen capture access: {screen_capture_access}");

    if !screen_capture_access {
        println!("Requesting screen capture access...");
        let result = permission::request_screen_capture_access();
        println!("Request result: {result}");
    }

    for window in window_getter::get_windows().unwrap() {
        println!("{:?}", window.title());
    }
}
