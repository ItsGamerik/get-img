# get-img: Discord image and message indexer

[![Build and test](https://github.com/ItsGamerik/get-img/actions/workflows/build.yml/badge.svg)](https://github.com/ItsGamerik/get-img/actions/workflows/build.yml)

## Description

Index attachments and messages (including threads!) in a discord channel or an entire server with a simple command and save the content to a file.
Once you have indexed messages, you can download the message list with a command, and you can also automatically download all the attachments that were uploaded to your computer!

## Usage

### Building

just use [cargo](https://www.rust-lang.org/tools/install) (the rust building utility) for building:

```shell
cargo build --release && cargo run --release
```

### Running

First, set your Discord bot token as an environment variable:

```shell
export DISCORD_TOKEN="<your_token_here>"
```

pro-tip: make sure to put a space before the "export" to hide the command from your history.

on windows, you can use the

```powershell
$ENV:DISCORD_TOKEN="<your_token_here>"
```

command to set your environment variable.

**You will have to do the same for the `GUILD_ID` environment variable so the bot can register all the commands to your Discord server.**

### Commands

| Command                        | Usage                                                                                                                                                                                               |
|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| /help                          | displays the help message.                                                                                                                                                                          |
| /index [channel]               | index every message with an attachment in a channel.                                                                                                                                                |
| /watch [channel] [bool] [bool] | toggles the automatic indexing for the specified channel on and off. Can also automatically download the attachments to disk if toggled on (optional).                                              |
| /download [bool]               | sends the index file into the discord channel, and will download attachments if specified.                                                                                                          |
| /indexall                      | index all messages of the server where the interaction was sent. Due to API limitations, this can take quite a long time, especially for larger servers. Progress is indicated by the bot's status. |

you have to be an administrator of the discord server you are using the bot in to be able to use the commands:  

- /index

- /download

- /watch

- /indexall

### Docker

(this still requires some editing)

Docker images are available [here](https://hub.docker.com/r/gamerik/get-img)

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
