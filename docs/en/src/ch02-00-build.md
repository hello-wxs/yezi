# Build

Before building them, make sure you have Rust installed.
If you don't have it installed, you can follow the [official Rust installation guide](https://www.rust-lang.org/tools/install) to install It.

## Build the Project

After installing Rust, you can build the project using the following command:

```shell
cargo build
```


## Build the Documentation

The documentation is built with [mdBook](https://rust-lang.github.io/mdBook/) witch made with rust.

If you don't have it installed, you can install it using the following command:

```shell
cargo install mdbook
```

After installing mdBook, you can build the documentation using the following command:

```shell
cd docs/en
mdbook build
```
or
```shell
cd docs/zh_cn
mdbook build
```
