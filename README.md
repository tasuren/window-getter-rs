# window-getter-rs

This is a Rust library for retrieving information about the windows open on the OS.
Currently, it only supports macOS and Windows.

[![Crates.io Version](https://img.shields.io/crates/v/window-getter)](https://crates.io/crates/window-getter)
[![docs.rs](https://img.shields.io/docsrs/window-getter)](https://docs.rs/window-getter/latest/window_getter/)

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

## Platform supports

- [x] macOS
- [x] Windows
- [ ] Linux?

I have no plans to make Linux support at this time due to my inexperienced knowledge about Linux.
But I'd be happy to receive pull requests.

### macOS permission

On macOS, you should need the permission of screen capture.
Otherwise, you can't get some window informations.

You can request the screen capture permission in runtime via this crate.
Example is [here](./examples/macos_permission.rs).
In development, you can also enable screen capture permission
for the apps used to run the project (such as terminal or editors).

## License

This project is licensed under the [MIT License](./LICENSE).
