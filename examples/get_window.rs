use window_getter::WindowId;

fn main() {
    let raw = std::env::var("WINDOW_ID")
        .expect("`WINDOW_ID` environment variable not set")
        .parse::<u32>()
        .expect("`WINDOW_ID` must be a valid `u32`");
    let id = WindowId::from(raw);

    if let Some(window) = window_getter::get_window(id).unwrap() {
        println!("title: {:?}", window.title());
    } else {
        println!("No window found with the given ID.");
    };
}
