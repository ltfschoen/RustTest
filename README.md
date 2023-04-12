README
============

# Table of Contents

  * [About](#about)
  * [Progress](#progress)
  * [Glossary](#glossary)
  * [Setup](#setup)
  * [Learning](#learning)
  * [Debugging](#debugging)
  * [Documentation](#documentation)
  * [Testing](#testing)
  * [Advanced](#advanced)

Note: Generate a new chapter with `openssl rand -hex 3`

## About <a id="about"></a>

Applied code using the book [The Rust Programming Language](http://doc.rust-lang.org/).

- [Rust is a general purpose, multi-paradigm, compiled programming language developed by Mozilla Research](http://en.wikipedia.org/wiki/Rust_%28programming_language%29).
- [Rust's goal is to provide the safety and convenience of modern programming languages, while still offering the efficiency and low-level control that C and C++ offer.](http://blog.rust-lang.org/2014/09/15/Rust-1.0.html).
- Rust is a systems language ([see slides](http://huonw.github.io/rust-sydney-feb15/)).

## Progress <a id="progress"></a>

* [x] 1st edition - Up to 4. III: Advanced Rust - 4.1 FFI
* [x] 2nd edition (re-learning) - Up to 5 Structs
* [x] 2nd edition (re-learning) - After 5 Structs
* [x] Convert Function and Variable names to snake case instead of camel case
* [x] Change to 4-space indentation
* [ ] [The Rust Programming Language](https://doc.rust-lang.org/book/)
  * [ ] [Quiz Version](https://rust-book.cs.brown.edu/)
  * [X] Chapters 1
  * [X] Chapters 2
  * [X] Chapters 3
  * [X] Chapters 4
    * [X] 4.1-4.3
    * [X] 4.4 Slices
  * [X] Chapters 5
    * [X] 5.1
    * [X] 5.2
    * [X] 5.3
  * [X] Chapters 6
    * [X] 6.1
    * [X] 6.2
    * [X] 6.3
  * [X] Chapters 7
  * [X] Chapters 8
  * [X] Chapters 9
    * [X] 9.1
    * [X] 9.2
    * [X] 9.3
  * [X] Chapters 10
    * [X] 10.1
    * [X] 10.2
    * [X] 10.3
  * [ ] Chapters 11
  * [ ] Chapters 13
    * [X] 13.1
    * [X] 13.2
    * [ ] 13.3
    * [ ] 13.4
  * [ ] Chapters 19
* [ ] Setup rustfmt/clippy
* [ ] Fix ./projects/privacy
* [ ] Fix ./projects/users
* [ ] Fix ./projects/unsorted
* N/A [The Rust Programming Language Book 2018 Edition (Second Edition deprecated)](https://github.com/rust-lang/book/blob/master/2018-edition)
* [ ] [The Cargo Book](https://doc.rust-lang.org/cargo/)
* [ ] [Rustlings](https://github.com/rust-lang/rustlings)
* [ ] [Rust and WebAssembly Book](https://rustwasm.github.io/book/)
* [ ] [Rust Course - Udemy](https://www.udemy.com/course/ultimate-rust-crash-course/)
* [ ] [Rust Course - Educative.io](https://www.educative.io/courses/ultimate-guide-to-rust-programming)
* [ ] [Rust Course - ZTM](https://zerotomastery.io/courses/learn-rust/)
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
* [ ] Read [The Rust Reference](http://doc.rust-lang.org/reference.html)
* [ ] Read [Rustonomicon (Advanced Rust)](https://doc.rust-lang.org/stable/nomicon/README.html)
* [ ] [Exercises with HashMaps](https://github.com/rust-lang/book/blob/master/2018-edition/src/ch08-03-hash-maps.md)

## Glossary <a id="glossary"></a>

Key Terminology of the Rust language and ecosystem:

  - **Cargo**: Package Manager for the Rust language for shipping Rust Crates to others. Cargo.toml configuration file defines dependencies and the build process.
  - **Crates.io**: Central location (similar to Node.js' [npmjs.org](npmjs.org) and Ruby's [rubygems.org](rubygems.org) that serve as one-stop-shops for Bundler dependencies) to publish, discover, and download Rust language Crates (Rust Libraries)
  - **Crates**: Libraries (aka Packages) that may either just produce an executable or a shared library. They are included as dependencies in Rust projects' Cargo.toml file. Crates may interoperate with other crates, so a Cargo manifest file declares the dependencies of each Crate. [Read more here](http://blog.rust-lang.org/2014/11/20/Cargo.html)
  - **Crate Root Module & Sub-Modules**: Code contents of a Crate may be Partitioned by defining a tree layout with a Root Module and Sub-Modules. Crates are defined in files with .rs file extension and build with Cargo to compiled crates with .rlib file extension. [Read more here](http://doc.rust-lang.org/book/crates-and-modules.html)
  - **Modules**: Rust's Module System is used to partition code (Modules) within a Crate. It facilitates the software engineering practice of splitting projects into smaller more manageable parts and with an interface whereby functionality may be public or private. Modules is defined with ```mod my_module``` (where ```my_module``` is the associated name identifer). Sub-modules (and Functions inside them) are defined and referred to by using the ```::``` notation (i.e. ```my_module::my_sub_module::my_function()```). Import External Crates (link separate files containing a Crate and its associated Modules) with ```extern crate my_module;```. Private Interfaces are the default in Rust. Modules and Functions may be exported as Public Interfaces instead by prefixing with the ```pub``` keyword (i.e. ```pub mod my_module``` or ```pub fn my_function```) [Read more here](http://smallcultfollowing.com/rust-int-variations/imem-umem/guide-crates.html)
  - **Rust [Releases Channel System](http://blog.rust-lang.org/2014/09/15/Rust-1.0.html)**: Nightly (latest updates including unstable features and libraries that may change), Beta (excludes unstable updates), Stable

## Setup <a id="setup"></a>

* [Minimum Rust Compiler version rustc 1.49.0-nightly](http://doc.rust-lang.org/book/installing-rust.html)
  ```
  rustc --version
  ```

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
    mkdir projects && cd projects;
    cargo new app_name --lib
    cargo build
    ./target/debug/app_name
    cargo doc --open
    ```

* **Build, and Run**
  * Clone or fork [RustTest repository](https://github.com/ltfschoen/RustTest.git)

  * Quick Cargo Check Script
    ```
    ./check_all.sh
    ```
    * Note: There is now a Cargo.toml workspace file in the project root directory, so it is possible to build all projects that are specified as member projects in that file from the project root directory and the output generated together in ./target/debug

  * Option #1 (Compile and Run)
    - Compile (generates an executable file 'main' in current directory). Since Rust is an ahead-of-time compiled language. Run (run the executable file 'main')
    ```
    rustc ./src/main.rs
    ./main
    ```

  * Option #2 (Separate Build and Run)
    - Build (uses Cargo.toml config file metadata and build requirements.
    - Coordinates build of complex projects having multiple crates
    - Assuming source code in ./src/hello_world.rs directory it generates binary executable file 'hello_world' and dependencies in ./target/debug/hello_world and generates Cargo.lock to track dependencies)
    ```
    cargo build
    ```
    - Note: There is a rust-toolchain file in the root of the repository, so it automatically checks the rust version specified in that file that is supported, and downloads, switches to, and installs that version, and it shows that new version when you run `rustup toolchain list`.
    - Note: To quickly check it compiles without building run `cargo check`
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

* **Fix linting errors**:
    ```
    cargo fix
    ```

* **Unit & Integration Tests**
    - Builds test runner binary to run functions annotated with
    the `test` attribute
    ```
    cargo test -- --nocapture
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
    * [Reference](https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html#updating-a-crate-to-get-a-new-version)

## Learning <a id="learning"></a>

### Guides

* [Play Rust](https://play.rust-lang.org/)
* [Rust Prelude](https://doc.rust-lang.org/std/prelude/index.html)
* [Rust Standard Library](http://doc.rust-lang.org/std/)
* [Rust By Example](http://rustbyexample.com/)
* [Programming Rust Book Example Code](https://github.com/ProgrammingRust/examples)
* [Rust Essentials Book Example Code](https://github.com/utilForever/RustEssentials)

### Libraries

* [Crates.io (Rust Community Crate Host)](https://crates.io/).
* [The Rust Reference](http://doc.rust-lang.org/reference.html)
* Rust CI (Github Actions. [Example](https://github.com/DataHighway-DHX/node/tree/master/.github/workflows))

### Community

* [Rust Community Forums](http://users.rust-lang.org/)
* [Rust on Reddit (Official Discussion Forums)](http://www.reddit.com/r/rust/)
* [Rust Weekly Community Blog](http://this-week-in-rust.org/)
* [Rust Sydney Meetup Slides by Huon](http://huonw.github.io/rust-sydney-feb15/)

## Debugging <a id="debugging"></a>

* Reference: https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits
* `println!` macro takes a reference of an expression and prints to standard output `stdout` and uses the `Display` default formatter when it uses `{}` for primitive types.
* `Debug` Trait output formatter may be used with `{:?}` to debug Structs after opting into
its usage by including the derive annotation attribute `#[derive(Debug)]` before the Struct definition (e.g. `println!("{}")` macro) since 
Use `{:#?}` for better readability.
* Alternative to print values using `Debug` format is using `dbg!` macro that prints to standard error console `stderr` and takes ownership of the expression and includes the line number, resultant value, and returns ownership of the value.
* Show debugging logs when running tests with `cargo test -- --nocapture`

### Debugging with Visual Studio Code

* Install [VS Code](https://code.visualstudio.com/download)
* Install rust libraries
  ```
  rustup component add rust-src
  rustup component add rustfmt
  rustup component add clippy
  ```
* Review
  * https://code.visualstudio.com/docs/languages/rust
  * https://rust-analyzer.github.io/manual.html
* Install VSCode Extensions in .vscode/extensions.json.
  * CTRL + P before entering `ext install <insert-extension>`. OR
  * Install from terminal:
    ```
    code --install-extension vadimcn.vscode-lldb
    code --install-extension rust-lang.rust-analyzer
    code --install-extension ms-vscode-remote.vscode-remote-extensionpack
    code --install-extension bungcip.better-toml
    ```
* Code > Preferences > Settings > debug.allowBreakpointsEverywhere > Enable
* Code > Preferences > Settings > debug.showBreakpointsInOverviewRuler > Enable
* Code > Preferences > Settings > editor.inlayHints.enabled > Enable
* Use [VSCode Logpoints](https://code.visualstudio.com/docs/editor/debugging#_logpoints)
* Formatter using rustfmt - SHIFT + OPTION + F
* Manual Code Suggestion - CTRL + SPACE
* Build All Projects in VSCode at Tasks > Run Build Task - SHIFT + CMD + B. Configure Task Config file **/.vscode/tasks.json**. Possible options https://gist.github.com/deadalusai/9e13e36d61ec7fb72148
  * Run binary from ./target/debug folder
* Debug Projects with Run & Debug > Choose "LLDB Debug - Program - ...". Configure Debug Config file **/.vscode/launch.json**.
* Note: If you get error `rust-analyzer failed to discover workspace`, it should be resolved following these steps https://stackoverflow.com/a/75649115/3208553. If not, try: Code > Preferences > Settings > rust-analyzer.linkedProjects. Update in settings.json

* Old links
  * Follow [this blog post](https://medium.com/@royalstream/rust-development-using-vs-code-on-os-x-debugging-included-bc10c9863777) to setup debugging with breakpoints using LLDB in VS Code.
  * Troubleshoot [by following these steps](https://medium.com/@ltfschoen/im-using-macos-10-12-861c8211006) 
  * Further information is [here](https://gitter.im/Drops-of-Diamond/Development?at=5aca2b7227c509a774ed9908). Use 

### Debugging with IntelliJ Rust Plugin

* https://intellij-rust.github.io/docs/quick-start.html

## Documentation <a id="documentation"></a>

* **Rust Compiler & Cargo**
  ```
  rustc --help
  cargo --help -v
  cargo doc --help
  ```

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

## Testing <a id="Testing"></a>

* Actions in test function body
  * Actual data or state setup
  * Expected data or state setup
  * Run code to test
  * Assert results expected

* Features for testing in Rust:
  * `test` attribute (metadata) annotation
  * `should_panic` attribute
  * `#[test]` to convert to test function on line before `fn`

## Advanced <a id="advanced"></a>

* [Pointers, Ownership & Lifetimes](http://paulkoerbitz.de/posts/Understanding-Pointers-Ownership-and-Lifetimes-in-Rust.html)
* [Pointers Cheatsheet](http://doc.rust-lang.org/book/pointers.html#cheat-sheet)
  - [Stack vs Heap](http://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap)
* [Building Static C Libraries](http://www.cs.dartmouth.edu/~campbell/cs50/buildlib.html)
* [C vs Rust Slides](http://www-verimag.imag.fr/~mounier/Enseignement/Software_Security/19RustVsC.pdf)
* [Seminar of Rust Programming Language](http://www8.cs.umu.se/kurser/5DV086/VT18/resources/seminar/rust.pdf)
* [Mozilla Research Projects including Servo](https://www.mozilla.org/en-US/research/projects/)

## Troubleshooting

* Running `cargo build` outputs: `Blocking waiting for file lock on build directory` 
  * Try disabling rust-analyser extension in VSCode, or in Ubuntu run: `rm ~/.cargo/.package-cache`
