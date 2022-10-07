fn main() {
    // add ayudame to library search path
    println!("Exectuing Build script");
    let lib_type = std::env::var("LIB_TYPE").unwrap_or("minimal".to_string());
    println!("cargo:rustc-link-search=/home/patrick/hlrs/rust_rewrite/installs/Ayudame/{}-no-ompt/lib/", lib_type);
    println!("cargo:rustc-link-lib=ayudame");  
}