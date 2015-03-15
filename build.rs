// build.rs

// Import dependency of externally maintained 'cc' package which manages
// invoking the C compiler.
extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("src/c/hello.c")
                .include("src")
                .compile("libhello.a");
}