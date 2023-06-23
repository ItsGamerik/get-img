# get-img: Discord image indexer

[![Build and test](https://github.com/ItsGamerik/get-img/actions/workflows/build.yml/badge.svg)](https://github.com/ItsGamerik/get-img/actions/workflows/build.yml)

## Description

Index images in a discord channel with a simple command and save the links to a file.

## Usage

### Building

just use `cargo` for building:

```shell
cargo build --release && cargo run --release
```

### Running

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

### Commands

```text
/hello                      say hello in a random language
/index [channel] [bool]     index every message with an attachment in a channel, unless "false" is used
/download                   download the links saved in the output.txt file
/watch [channel] [bool]     toggles the automatic indexing for a single channel on and off, can only be ON for one channel at a time.
```

## Setting up a bot through Discord

1. Go to the [Discord Developer page](https://discord.com/developers/applications).

2. Click on `New Application`, give it a name and register it to yourself.

3. Read the TOS and Developer Policy and agree. Hit `Create`.

4. Select your new App and open the `OAuth2` Section. Reset your `Client Secret` and make sure to write it down in a safe place.

5. Now head to the OAuth2 section `URL Generator`. Select `bot` as the scope and add the necessary permissions.

6. Copy the link to your clipboard but don't open it yet.

7. Click on the section `Bot` and uncheck the switch called `Public Bot`

8. Scroll down to the switch called `Message Content Intent`. Check the switch. This will be necessary to read the message history in channels.

9. Now paste the link into your browser and add the bot to your discord server. If everything worked, you should now be able to use the bot!

## Additional useful links

- API reference for interactions
  - [application command reference](https://discord.com/developers/docs/interactions/application-commands)
  - [general discord api reference](https://discord.com/developers/docs/reference)
