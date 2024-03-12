use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("hello.rs");
    let dest_path = Path::new("./src").join("hello.rs");

    println!("totototo");
    println!("cargo:warning=your text");

    println!("cargo:warning= {:?}", dest_path.to_str());
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        "
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}