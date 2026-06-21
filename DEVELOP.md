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

This project set many alias for you to test it quickly. Here are the uese.

1. `cargo dbg` -> `cargo run -p yezi-dbg`
2. `cargo tui` -> `cargo run -p yezi-tui`
3. `cargo dev` -> `cargo run -p yezi-dev -- jump`

Besides, doc includes private items.

Click [here](./.cargo/config.toml) for the primary file.

## Development tool in this project

You can see there is a crate named yezi-dev. It's a crate used for build test environment. You can use `cargo dev` to compile it and use commands like `dev seetup` to use it. If you want see more, you can see the [sources](./crates/yezi-dev/src/main.rs).

## The dbg crate

The dbg crate is used to test the lib crate. Besides, **don't commit it**!!
