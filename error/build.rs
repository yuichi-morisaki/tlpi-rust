fn main() {
    cc::Build::new()
        .file("src/error_text.c")
        .compile("error");
    println!("cargo:rerun-if-changed=src/error.c");
}
