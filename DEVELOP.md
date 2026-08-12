## Development Requirements

1. In line with the best engineering practices.
2. Use Rust as much as possible.
3. All of the commits must pass these requires:

```shell
cargo check
cargo fmt
cargo clippy
```

## Command alias

This project set many alias for you to test it quickly.
Here are the uses.
Click [here](./.cargo/config.toml) for the primary file.

## Development tool in this project

You can see there is a crate named yezi-dev. It's a crate used for build test environment. You can use `cargo dev` to compile it and use commands like `dev seetup` to use it. If you want see more, you can see the [sources](./crates/yezi-dev/src/main.rs).

## The dbg crate

The dbg crate is used to test the lib crate.
<p color="red">
***DON'T COMMIT THE CHANGE OF IT!!***
</p>
