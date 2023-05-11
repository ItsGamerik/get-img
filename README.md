# get-img: Discord image indexer

[![build](https://github.com/ItsGamerik/get-img/actions/workflows/rust.yml/badge.svg)](https://github.com/ItsGamerik/get-img/actions)

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

optionally, you can also set a `GUILD_ID` as an environment variable to use the `/hello` command.

### commands

```text
/hello                      say hello in a random language
/index [channel] [bool]     index every message with an attachment in a channel, unless "false" is used
/download                   download the links saved in the output.txt file
```
