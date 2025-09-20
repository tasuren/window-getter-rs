# window-getter-rs
This is a Rust library for retrieving information about the windows open on the OS.  
Currently, it only supports macOS and Windows.

## Examples
```rust
fn main() {
    let windows = window_getter::get_windows().unwrap();

    for window in windows {
        if let Ok(Some(title)) = window.title() {
            println!("{title}");
        }
    }
}
```

## macOS supports
On macOS, you should need the permission of screen capture.
Otherwise, you can't get some window informations.

You can request the screen capture permission in runtime via this crate.
Example is [here](./examples/macos_permission.rs).  
In development, you can also enable screen capture permission
for the apps used to run the project (such as terminal or editors).

## ToDo
First release todos:
- [x] macOS
  - [x] Get all windows
  - [x] Window bounds
  - [x] Window title
  - [x] PID of owner that has Window
- [x] Windows
  - [x] Get all windows
  - [x] Window bounds
  - [x] Window title
  - [x] PID of owner that has Window
- [ ] Linux?
  I have no plans to make this at this time due to my inexperienced knowledge about Linux.  
  But I'd be happy to receive pull requests.

## License
This project is licensed under the [MIT License](./LICENSE).
