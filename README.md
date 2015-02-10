README
============

Applied code using the book [The Rust Programming Language](http://doc.rust-lang.org/)

Requirements
-------

* [Rust Compiler Version 1.0.0-nightly](http://doc.rust-lang.org/book/installing-rust.html)
  ```
    rustc --version
  ```

Setup
-------

1. **Setup RustTest**
  * Clone or fork [RustTest repository](https://github.com/ltfschoen/RustTest.git)
  * Option #1 (Compile and Run)
    - Compile (generates an executable file 'main' in current directory)
    ```
    rustc ./src/main.rs
    ```
    - Run (run the executable file 'main')
    ```
    ./main
    ```
  * Option #2 (Build and Run)
    - Build (in accordance with the Cargo.toml configuration file's metadata and build outcome requirements, and assuming the source code is in the /src directory it generates a binary executable file 'hello_world' and dependencies in /target folder of current directory, and generates Cargo.lock to track dependencies)
    ```
    cargo build
    ```
    - Run (run the executable file 'hello_world')
    ```
    ./target/hello_world
    ```

Other Links
-------

* [Play Rust](https://play.rust-lang.org/)
  - Click "Evaluate" to run code

Help
-------

```
rustc --help
cargo --help -v
```