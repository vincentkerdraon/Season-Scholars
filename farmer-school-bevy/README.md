# Season Scholars - Rust version

## Startup options

See env var for fullscreen, volume, ...

## Run from code

_(requires all the dependencies...)_\
see `farmer-school-bevy/.vscode/launch.json`

```
SEASON_SCHOLARS_DEV_ASSETS=y cargo run
```

## Release

- update version in `Cargo.toml`
- update version in `src/model/config.rs`
- build
```
cargo build --target=x86_64-unknown-linux-gnu --release
cargo build --target=armv7-unknown-linux-gnueabihf --release
cargo build --target=aarch64-unknown-linux-gnu --release
cargo build --target=x86_64-pc-windows-gnu --release
cargo build --target=x86_64-pc-windows-msvc --release
cargo build --target=x86_64-apple-darwin --release
cargo build --target=aarch64-apple-darwin --release
```
- prepare zip in ../release
- push
- create gitlab release with link to zip

## Note

First medium project in rust + bevy.
- resolution is fixed
- bad error management
- linux and windows only