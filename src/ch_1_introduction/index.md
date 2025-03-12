# Quick Info

- **`rustc`** (The Compiler)
- **`cargo`** (The Build System & Package Manager)
- **`rustup`** (The Toolchain Manager)

> [!NOTE]
> `rustup update` updates **both `rustc` and `cargo`** (and the **standard library**) for **all installed toolchains**.

> [!EXAMPLE]
> ```bash
> rustup toolchain list
> # stable-x86_64-apple-darwin (default)
> # nightly-x86_64-apple-darwin
>
> rustup update
> # updates both stable and nightly toolchains
>
> rustup self uninstall
> # uninstalls rustup and all installed toolchains
>
> rustup install nightly
> # installs the nightly toolchain
> 
> rustup default nightly
> # sets the default toolchain to nightly
>
> rustup doc
> # opens the rust documentation
> ```

---

### Rustc (The Compiler)
- **Function:** Takes Rust source code (`.rs` files) and compiles it into machine code (executables or libraries).
- **Usage:** `rustc file.rs`

### Rustup (The Toolchain Manager)
- **Function:** Installs and manages complete Rust toolchains, including `rustc`, `cargo`, and the standard libraries.
- **Version Management:** Allows installing and switching between different release channels (`stable`, `beta`, `nightly`) and specific versions.
- **Updates:** Simplifies updating your Rust installation using `rustup update`.
- **Installation:** The primary and recommended way to install Rust on most systems.

### Cargo (The Build System & Package Manager)
- **Function:** Manages building, testing, and running Rust projects, and handles dependency management.
- **Project Management:** Uses `Cargo.toml` to define project metadata, dependencies, and build configuration.
- **Common Commands:**
  - `cargo build` – Compile the project
  - `cargo run` – Build and run the project
  - `cargo test` – Run tests
  - `cargo check` – Quickly check code for errors without producing binaries
  - `cargo build --example <example_name>` – Build a specific example
  - `cargo run --example <example_name>` – Build and run a specific example
- **Ecosystem:** Integrates with **crates.io**, Rust’s official package registry.
- **Abstraction:** Calls `rustc` under the hood, so developers rarely need to interact with the compiler directly.


## Running a Rust Program

```sh
mkdir "%USERPROFILE%\projects"
cd /d "%USERPROFILE%\projects"
mkdir hello_world
cd hello_world
```
create filename: main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```
Save the file and go back to your terminal window in the ~/projects/hello_world directory. On Linux or macOS, enter the following commands to compile and run the file

```sh
rustc main.rs
./main
Hello, world!
```

## Running a Rust Program (Using Cargo)

```sh
cargo new hello_cargo
cd hello_cargo
```
create filename: main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```
This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug

```sh
cargo build
  Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
  Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
If all goes well, Hello, world! should print to the terminal. Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock. This file keeps track of the exact versions of dependencies in your project

```sh
./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use cargo run to compile the code and then run the resultant executable all in one command

```sh
cargo run
  Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
  Running `target/debug/hello_cargo`
Hello, world!
```

Building for Release
When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in `target/release` instead of `target/debug`. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile.