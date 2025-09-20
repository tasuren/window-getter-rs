fn main() {
    let windows = window_getter::get_windows().unwrap();
    println!("Found {} windows:", windows.len());

    for window in windows {
        println!("\n{:?} ({})", window.title(), window.id().as_u32());
        println!("\tBounds: {:?}", window.bounds());
        println!("\tProcess id: {}", window.owner_pid().unwrap());
        println!("\tProcess name: {:?}", window.owner_name());
    }
}
