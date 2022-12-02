# 🎄🎁[advent-of-code-2022](https://adventofcode.com/)🎁🎄 (in Rust🦀!)

## First Time Set Up
### Mac/Linux
The preferred way to install rust is with [Rustup](https://www.rust-lang.org/tools/install)
```bash 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installation, run ```rustc --version``` - if it fails, you may want to look into including ```~/.cargo/bin``` to your PATH variable

### Windows
Download and run [rustup-init.exe](https://forge.rust-lang.org/infra/other-installation-methods.html#:~:text=download%20and%20run-,rustup%2Dinit.exe,-.) - if the download link doesn't work, [check here for more info](https://forge.rust-lang.org/infra/other-installation-methods.html)

## Working with Rust
### Writing and Running a Rust Program
Rust files end with the .rs extension. If you just want to get started, create a file called main.rs as follows:
```rust
fn main() {
    println!("Hello, world!");
}
```
and then compile with ```rustc main.rs``` and run with ```./main``` (or ```.\main.exe``` on Windows)

### Cargo
When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:

- build your project with ```cargo build```
- run your project with ```cargo run```
- test your project with ```cargo test```
- build documentation for your project with ```cargo doc```
- publish a library to crates.io with ```cargo publish```
To test that you have Rust and Cargo installed, you can run this in your terminal of choice: ```cargo --version```

# Resources
- [Rust Lang Book](https://doc.rust-lang.org/book/title-page.html)
- [Learn Rust ](https://www.rust-lang.org/learn)
- [Rust Documentation](https://doc.rust-lang.org/beta/)
- [Rust By Example](https://doc.rust-lang.org/beta/rust-by-example/meta/doc.html)
