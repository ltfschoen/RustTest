// build.rs

// Import dependency of externally maintained 'cc' package which manages
// invoking the C compiler.
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c/hello.c")
        .include("src")
        .compile("libhello.a");
}