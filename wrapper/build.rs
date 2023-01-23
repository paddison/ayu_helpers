fn main() {
    // add ayudame to library search path
    println!("Exectuing Build script");
    let _lib_type = std::env::var("LIB_TYPE").unwrap_or("minimal".to_string());
    // println!("cargo:rustc-link-search=/home/patrick/hlrs/rust_rewrite/installs/Ayudame/full/lib");
    // println!("cargo:rustc-link-lib=dylib=ayudame");  
}