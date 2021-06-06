# edsdk-rs
EDSDK for rust

All of the EDSDK's functions have not been checked yet.

# usage
Cargo.toml
```
edsdk = { git = "https://github.com/bob-yamaguchi/edsdk-rs/", branch = "master"}
```
## rust wrapper
```rust
use edsdk::wrap::*;

fn main() {
    let library = Library::initialize().unwrap();
    let device_list = library.get_device_list();
    for device in device_list{
        let device_info = device.get_device_info();
        // or do something
    }
}
```


