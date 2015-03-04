README
============

[![Build Status](https://api.travis-ci.org/ltfschoen/RustTest.svg)](https://travis-ci.org/ltfschoen/RustTest)

About
-------

Applied code using the book [The Rust Programming Language](http://doc.rust-lang.org/).

[Rust is a general purpose, multi-paradigm, compiled programming language developed by Mozilla Research](http://en.wikipedia.org/wiki/Rust_%28programming_language%29). [Rust's goal is to provide the safety and convenience of modern programming languages, while still offering the efficiency and low-level control that C and C++ offer.](http://blog.rust-lang.org/2014/09/15/Rust-1.0.html). Rust is a systems language ([see slides](http://huonw.github.io/rust-sydney-feb15/)).

Key Terminology of the Rust language and ecosystem:

  - **Cargo**: Package Manager for the Rust language for shipping Rust Crates to others. Cargo.toml configuration file defines dependencies and the build process.
  - **Crates.io**: Central location (similar to Node.js' [npmjs.org](npmjs.org) and Ruby's [rubygems.org](rubygems.org) that serve as one-stop-shops for Bundler dependencies) to publish, discover, and download Rust language Crates (Rust Libraries)
  - **Crates**: Libraries (aka Packages) that may either just produce an executable or a shared library. They are included as dependencies in Rust projects' Cargo.toml file. Crates may interoperate with other crates, so a Cargo manifest file declares the dependencies of each Crate. [Read more here](http://blog.rust-lang.org/2014/11/20/Cargo.html)
  - **Crate Root Module & Sub-Modules**: Code contents of a Crate may be Partitioned by defining a tree layout with a Root Module and Sub-Modules. Crates are defined in files with .rs file extension and build with Cargo to compiled crates with .rlib file extension. [Read more here](http://doc.rust-lang.org/book/crates-and-modules.html)
  - **Modules**: Rust's Module System is used to partition code (Modules) within a Crate. It facilitates the software engineering practice of splitting projects into smaller more manageable parts and with an interface whereby functionality may be public or private. Modules is defined with ```mod my_module``` (where ```my_module``` is the associated name identifer). Sub-modules (and Functions inside them) are defined and referred to by using the ```::``` notation (i.e. ```my_module::my_sub_module::my_function()```). Import External Crates (link separate files containing a Crate and its associated Modules) with ```extern create my_module;```. Private Interfaces are the default in Rust. Modules and Functions may be exported as Public Interfaces instead by prefixing with the ```pub``` keyword (i.e. ```pub mod my_module``` or ```pub fn my_function```) [Read more here](http://smallcultfollowing.com/rust-int-variations/imem-umem/guide-crates.html)
  - **Rust [Releases Channel System](http://blog.rust-lang.org/2014/09/15/Rust-1.0.html)**: Nightly (latest updates including unstable features and libraries that may change), Beta (excludes unstable updates), Stable

Progress
-------

Up to 3. II: Intermediate Rust - 3.3 Tests - The test module

Requirements
-------

* [Rust Compiler Version 1.0.0-nightly](http://doc.rust-lang.org/book/installing-rust.html)
  ```
    rustc --version
  ```

Setup
-------

1. **Update Rust** Nightly build and Cargo (Rust's Package Manager). Perform this regularly after [Rust installed](rust-lang.org))
  ```
  curl -sS https://static.rust-lang.org/rustup.sh | sudo bash
  ```

2. **Build, and Run**
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
  * Option #2 (Separate Build and Run)
    - Build (in accordance with the Cargo.toml configuration file's metadata and build outcome requirements, and assuming the source code is in the /src directory it generates a binary executable file 'hello_world' and dependencies in /target folder of current directory, and generates Cargo.lock to track dependencies)
    ```
    cargo build
    ```
    - Run (run the executable file 'hello_world')
    ```
    ./target/hello_world
    ```
  * Option #3 (Combined Build and Run)
    - Build and Run (combined with [Cargo](http://doc.crates.io/guide.html))
    ```
    cargo run
    ```

3. **Unit & Integration Tests**
    ```
    cargo test
    ```

4. **Benchmark Tests**
    ```
    cargo bench
    ```

Rust Docs
-------

* Refer to separate branch [feature/rustdocs](https://github.com/ltfschoen/RustTest/compare/feature/rustdocs?expand=1) for attempt at using rustdocs (unsuccessful as no HTML file was generated in the /docs/ directory when ```rustdoc ./src/main.rs``` was run, and no tests are run when ```rustdoc --test ./src/main.rs``` is run)

Other Links
-------

* [Rust CI Dashboard & Rust Projects](http://www.rust-ci.org/)
* [Rust Community Forums](http://users.rust-lang.org/)
* [Rust Standard Library](http://doc.rust-lang.org/std/)
* [Rust on Reddit (Official Discussion Forums)](http://www.reddit.com/r/rust/)
* [Play Rust](https://play.rust-lang.org/)
  - Click "Evaluate" to run code
* [Crates.io (Rust Community Crate Host)](https://crates.io/).
* [Rust Sydney Meetup Slides by Huon](http://huonw.github.io/rust-sydney-feb15/)
* [Pointers Cheatsheet](http://doc.rust-lang.org/book/pointers.html#cheat-sheet)
  - [Stack vs Heap](http://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap)


Help
-------

```
rustc --help
cargo --help -v
```