# get-img: Discord image indexer

[![build](https://github.com/ItsGamerik/get-img/actions/workflows/rust.yml/badge.svg)](https://github.com/ItsGamerik/get-img/actions)

## description

Index images in a discord channel with a simple command and save the links to a file.

## commands

```
/index      index every message with an attachment in a channel
@mention    legacy command system, not supported.
```

## usage

just use `cargo`:
```
cargo build --release && cargo run --release
```

### dependencies

- serenity
- tokio
