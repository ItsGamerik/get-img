# get-img: Discord image indexer

[![Build and test](https://github.com/ItsGamerik/get-img/actions/workflows/build.yml/badge.svg)](https://github.com/ItsGamerik/get-img/actions/workflows/build.yml)

## description

Index images in a discord channel with a simple command and save the links to a file.

## usage

### building

just use `cargo` for building:

```shell
cargo build --release && cargo run --release
```

### running

First, set your Discord bot token as an environment variable:

```shell
export DISCORD_TOKEN=""
```

pro tip: make sure to put a space before the "export" coma to hide the command from your history.

on windows you can use the

```powershell
$ENV:DISCORD_TOKEN=""
```

command to set your environment variable.

### commands

```text
/hello                      say hello in a random language
/index [channel] [bool]     index every message with an attachment in a channel, unless "false" is used
/download                   download the links saved in the output.txt file
/watch [channel] [bool]     toggles the automatic indexing for a single channel on and off, can only be ON for one channel at a time.
```
