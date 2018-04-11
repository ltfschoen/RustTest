README
============

[![Build Status](https://api.travis-ci.org/ltfschoen/RustTest.svg)](https://travis-ci.org/ltfschoen/RustTest) (Note: Travis-CI builds failing due to use of FFI, however the code compiles successfully when run locally)

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

* 1st edition - Up to 4. III: Advanced Rust - 4.1 FFI
* 2nd edition (re-learning) - Up to 5 Structs

TODO
-------

* [x] Convert Function and Variable names to snake case instead of camel case
* [x] Change to 4-space indentation
* [ ] [The Rust Programming Language Book 2018 Edition (Second Edition deprecated)](https://github.com/rust-lang/book/blob/master/2018-edition)
* [ ] [The Cargo Book](https://doc.rust-lang.org/cargo/)
* [x] Read [Ownership, Meta, Borrowing, Lifetimes, Shared Ownership](http://doc.rust-lang.org/book/ownership.html)
* [ ] Create Examples showing "immutability", "orthogonality" and "idempotence" trait importance
* [ ] Read [Cyclone Regions](http://www.cs.umd.edu/projects/cyclone/papers/cyclone-regions.pdf)
* [ ] Try different [Iterators in std::iter Module](http://doc.rust-lang.org/std/iter/)
* [ ] Read [Macro's Reference Docs](http://doc.rust-lang.org/reference.html#macros)
* [ ] Read [Advanced Macros](http://doc.rust-lang.org/book/advanced-macros.html)
* [ ] Read [Macros Repetition - Macro-by-Example](http://www.cs.indiana.edu/ftp/techreports/TR206.pdf)
* [ ] Read [Hygienic Macro System](http://en.wikipedia.org/wiki/Hygienic_macro)
* [ ] Fix [Rust Documentation](http://doc.rust-lang.org/book/documentation.html)
  - Only documentation for the Library is being generated, not the Binary
* [ ] Fix Travis-CI Build Error ```error: linking with `cc` failed: exit code: 1``` caused by using FFI (although the code compiles when run locally, Travis-CI does not like it)
* [ ] Read [The Rust Reference](http://doc.rust-lang.org/reference.html)
* [ ] Read [Rustonomicon (Advanced Rust)](https://doc.rust-lang.org/stable/nomicon/README.html)
* [ ] [Exercises with HashMaps](https://github.com/rust-lang/book/blob/master/2018-edition/src/ch08-03-hash-maps.md)

Requirements
-------

* [Rust Compiler Version 1.0.0-nightly](http://doc.rust-lang.org/book/installing-rust.html)
  ```
    rustc --version
  ```

IntelliJ Rust Plugin
--------------------

* https://intellij-rust.github.io/docs/quick-start.html

Setup
-------

* [Install Rust](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)

* **Update Rust**
  ```
  rustup update
  ```

* **Create Application**

  * Note: Cargo generates Cargo.toml, src/main.rs, and initialises Git repo

  * Binary Executable
    ```
    mkdir projects && cd projects;
    cargo new app_name --bin && cd app_name
    cargo build
    ./target/debug/app_name
    cargo doc --open
    ```

  * Library
    ```
    cargo new app_name --lib
    ```

* **Build, and Run**
  * Clone or fork [RustTest repository](https://github.com/ltfschoen/RustTest.git)

  * Option #1 (Compile and Run)
    - Compile (generates an executable file 'main' in current directory). Since Rust is an ahead-of-time compiled language.
    ```
    rustc ./src/main.rs
    ```
    - Run (run the executable file 'main')
    ```
    ./main
    ```

  * Option #2 (Separate Build and Run)
    - Build (uses Cargo.toml config file metadata and build requirements.
    - Coordinates build of complex projects having multiple crates
    - Assuming source code in ./src/hello_world.rs directory it generates binary executable
    file 'hello_world' and dependencies in ./target/debug/hello_world and generates Cargo.lock to track dependencies)
    ```
    cargo build
    ```
    - Run (run the executable file 'hello_world')
    ```
    ./target/debug/hello_world
    ```
  * Option #3 (Combined Build and Run)
    - Build and Run (combined with [Cargo](http://doc.crates.io/guide.html))
    ```
    cargo run

    cd ./projects/guessing_game/ && cargo run
    ```

* **Check Compilation of code without running the extended Build**
    ```
    cargo check
    ```

* **Unit & Integration Tests**
    - Builds test runner binary to run functions annotated with
    the `test` attribute
    ```
    cargo test
    ```

* **Benchmark Production Release with Tests**
    ```
    cargo bench
    ```

* **Production Release with Optimisations**
    - Build executable in ./target/release/hello_world
    ```
    cargo build --release
    ```

* **Updating Crates**
    - Ignores Cargo.lock
    - Finds latest versions of dependencies in Cargo.toml
    ```
    cargo update
    ```

Example Projects
---------------

* Guessing Game

    * Execution
        ```
        cd projects/guessing_game;
        cargo build;
        ./target/debug/guessing_game;
        cargo doc --open;
        cargo test;
        ```

    * Tests
        ```
        cd projects/guessing_game;
        cargo build;
        cargo test;
        ```

* Find a Word (originally setup with `mkdir projects && cd projects && cargo new find_word --bin && cd find_word;`)

    * Execution
        ```
        cd projects/find_word;
        cargo build;
        ./target/debug/find_word;
        cargo doc --open;
        ```

    * Tests
        ```
        cd projects/find_word;
        cargo build;
        cargo test;
        ```

* Shapes (originally setup with `mkdir projects && cd projects && cargo new shapes --bin && cd shapes;`)

    * Execution
        ```
        cd projects/shapes;
        cargo build;
        ./target/debug/shapes;
        cargo doc --open;
        ```

    * Tests
        ```
        cd projects/shapes;
        cargo build;
        cargo test;
        ```

* Users (originally setup with `mkdir projects && cd projects && cargo new users --bin && cd users;`)

    * Execution
        ```
        cd projects/users;
        cargo run;
        cargo doc --open;
        ```

    * Tests
        ```
        cd projects/users;
        cargo build;
        cargo test;
        ```

* Communicator Library (setup with `mkdir projects && cd projects && cargo new communicator --lib && cd communicator;`)

    * Note: Since it's a library is has lib.rs and we only build it

    * Execution
        ```
        cd projects/communicator;
        cargo build;
        cargo doc --open;
        ```

* Generics

    * Execution
        ```bash
        cd projects/generics;
        cargo run;
        ```

Rust Testing
------------
* Actions in test function body
    * Actual data or state setup
    * Expected data or state setup
    * Run code to test
    * Assert results expected

* Features for testing in Rust:
    * `test` attribute (metadata) annotation
    * `should_panic` attribute
    * `#[test]` to convert to test function on line before `fn`

Rust Docs
-------

* **Rust Language Book Offline**
    ```
    rustup doc
    ```

* **Library Documentation**
    - Build documentation provided by all local dependencies and open in browser
    ```
    cargo doc --open
    ```
    OR
    ```
    cargo doc --package hello_world --open
    ```

* **Binary Documentation**
  - TODO

* Note: Currently only the hello_world Library is being included in Rust Documentation
without the hello Library

* Refer to separate branch [feature/rustdocs](https://github.com/ltfschoen/RustTest/compare/feature/rustdocs?expand=1) for attempt at using rustdocs (unsuccessful as no HTML file was generated in the /docs/ directory when ```rustdoc ./src/main.rs``` was run, and no tests are run when ```rustdoc --test ./src/main.rs``` is run)

Other Links
-------

* [Rust CI Dashboard & Rust Projects](http://www.rust-ci.org/)
* [Rust Community Forums](http://users.rust-lang.org/)
* [Rust Standard Library](http://doc.rust-lang.org/std/)
* [Rust on Reddit (Official Discussion Forums)](http://www.reddit.com/r/rust/)
* [Rust Weekly Community Blog](http://this-week-in-rust.org/)
* [Play Rust](https://play.rust-lang.org/)
  - Click "Evaluate" to run code
* [Crates.io (Rust Community Crate Host)](https://crates.io/).
* [Rust Sydney Meetup Slides by Huon](http://huonw.github.io/rust-sydney-feb15/)
* [Pointers, Ownership & Lifetimes](http://paulkoerbitz.de/posts/Understanding-Pointers-Ownership-and-Lifetimes-in-Rust.html)
* [Pointers Cheatsheet](http://doc.rust-lang.org/book/pointers.html#cheat-sheet)
  - [Stack vs Heap](http://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap)
* [Mozilla Research Projects including Servo](https://www.mozilla.org/en-US/research/projects/)
* [Rust By Example](http://rustbyexample.com/)
* [Building Static C Libraries](http://www.cs.dartmouth.edu/~campbell/cs50/buildlib.html)
* [The Rust Reference](http://doc.rust-lang.org/reference.html)
* [Programming Rust Book Example Code](https://github.com/ProgrammingRust/examples)
* [Rust Essentials Book Example Code](https://github.com/utilForever/RustEssentials)
* [C vs Rust Slides](http://www-verimag.imag.fr/~mounier/Enseignement/Software_Security/19RustVsC.pdf)
* [Seminar of Rust Programming Language](http://www8.cs.umu.se/kurser/5DV086/VT18/resources/seminar/rust.pdf)

Debugging
---------

 * `println!` macro uses `Display` default formatter when it uses `{}` for primitive types.
 * `Debug` Trait output formatter may be used with `{:?}` to debug Structs after opting into
  its usage by including the derive annotation `#[derive(Debug)]` before the Struct definition.
  Use `{:#?}` for better readability.

Debugging with Visual Studio Code
---------

* Install [VS Code](https://code.visualstudio.com/download)
* Follow [this blog post](https://medium.com/@royalstream/rust-development-using-vs-code-on-os-x-debugging-included-bc10c9863777) to setup debugging with breakpoints using LLDB in VS Code.
* Troubleshoot [by following these steps](https://medium.com/@ltfschoen/im-using-macos-10-12-861c8211006) 
* Example Debug Config file **<PROJECT_NAME>/.vscode/launch.json**. Use by going to menu: Debug > Start Debugging. * Build any changes in terminal with `cargo build`
* Further information is [here](https://gitter.im/Drops-of-Diamond/Development?at=5aca2b7227c509a774ed9908). Use CTRL+P before entering `ext install <insert-extension>`.
```
{
  // Use IntelliSense to learn about possible attributes.
  // Hover to view descriptions of existing attributes.
  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug - Insert Program Name",
      "program": "${workspaceFolder}/target/debug/program_name",
      "args": "",
      "cwd": "${workspaceFolder}",
      "terminal": "integrated"
    }
  ]
}
```

* Example Build Task file **<PROJECT_NAME>/.vscode/tasks.json**. Use by going to menu: Tasks > Run Build Task
```
{
  // See https://go.microsoft.com/fwlink/?LinkId=733558
  // for the documentation about the tasks.json format
  "version": "2.0.0",
  "tasks": [
    {
      "label": "cargo build",
      "type": "shell",
      "command": "cargo build",
      "group": {
        "kind": "build",
        "isDefault": true
      }
    }
  ]
}
```

Help
-------

```
rustc --help
cargo --help -v
cargo doc --help
```
