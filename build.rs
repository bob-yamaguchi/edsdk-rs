use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-search=native=./lib");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_dll_path = Path::new(&out_dir).join("../../..").join("EDSDK.dll");
    let _ = fs::copy("./dll/EDSDK.dll", dest_dll_path);
}
