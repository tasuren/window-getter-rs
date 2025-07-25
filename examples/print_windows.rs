fn main() {
    let windows = window_getter::get_windows().unwrap();
    println!("Found {} windows:", windows.len());

    for window in windows {
        println!("\n{:?} ({})", window.title(), window.id().as_u32());
        println!("\tBounds: {:?}", window.bounds());
        println!("\tProcess ID: {}", window.owner_pid().unwrap());
        println!("\tProcess Name: {:?}", window.owner_name());
    }
}
