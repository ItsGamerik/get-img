# get-img: Discord image and message indexer
(maybe i'll continue working on this on Codeberg)

[![Build and test](https://github.com/ItsGamerik/get-img/actions/workflows/build.yml/badge.svg)](https://github.com/ItsGamerik/get-img/actions/workflows/build.yml)

## Description

This tool helps you preserve your Discord conversations and message attachments.
Capture and maintain the contents of your Discord server or specific channels using simple commands.
Index and download messages, attachments, and threads to save the entire contents to a file for easy backups or archiving.

## Usage
  
### Building

> instructions for using docker can be found [here](https://github.com/ItsGamerik/get-img#docker)

for compiling you will need the OpenSSL development package:
```shell
# for deb-based distros
$ sudo apt install libssl-dev

# for rpm-based distros
$ sudo dnf install openssl-devel 
```

just use [cargo](https://www.rust-lang.org/tools/install) (the rust building utility) for building:

```shell
$ cargo build --release
```

the executable can then be found in the folder `target/release`.

### Running

First, set your Discord bot token and guild ID as an environment variable:

```shell
$  export DISCORD_TOKEN="<your_token_here>"
$ export GUILD_ID="<your_gid_here>"
```

> pro-tip: make sure to put a space before the "export" command to hide your bot token from the shell history.

on **windows**, you can use the

```powershell
$ENV:DISCORD_TOKEN="<your_token_here>"
$ENV:GUILD_ID="<your_gid_here>"
```

command to set your environment variable.


then, run the bot:

```shell
$ cargo run --release
```

(or just run the executable)

**if you encounter any bugs or unexpected behaviour, make sure to open an issue on GitHub!**

### Commands

(these commands have to be run on the discord server where the bot is running)

| Command                        | Usage                                                                                                                                                                                                        |
|--------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| /help                          | displays the help message.                                                                                                                                                                                   |
| /index [channel]               | index every message with an attachment in a channel.                                                                                                                                                         |
| /watch [channel] [bool] [bool] | toggles the automatic indexing for the specified channel on and off. Can also automatically download the attachments to disk if toggled on (optional).                                                       |
| /download [bool]               | sends the index file into the discord channel, and will download attachments if specified.                                                                                                                   |
| /indexall                      | index all messages of the server where the interaction was sent. Due to API limitations, this can take quite a long time, especially for larger servers. Progress is indicated by the the status of the bot. |

you have to be an administrator of the discord server you are using the bot in to be able to use these commands:  

- /index

- /download

- /watch

- /indexall

### Docker

**Pre-built Docker images are available [on the Docker Hub](https://hub.docker.com/r/gamerik/get-img) or the [Packages](https://github.com/ItsGamerik?tab=packages&repo_name=get-img) page on GitHub**

You can also use docker to run the bot. Make sure to set the environment variables as described above.

to build the docker image, you will have to run
```shell
# using docker
$ docker build . --tag=<your_tag>

# using podman
$ podman build . --tag=<your_tag>
```
> The executable can be found in the directory `/usr/get-img/get-img`.

this will create two images, one for a builder helper and one for the actual application.
you can delete the builder helper image once the build process has completed. 

to run a container, use the following command:
```shell
# using docker
$ docker run -it -e "DISCORD_TOKEN=<your_token_here>" -e "GUILD_ID=<your_gid_here>" get-img:<version>

# using podman
$ podman run -it -e "DISCORD_TOKEN=<your_token_here>" -e "GUILD_ID=<your_gid_here>" get-img:<version>
```
> The downloaded files can be found in `/usr/get-img/download`.

## Setting up a bot through the Discord Developer Portal

1. Go to the [Discord Developer page](https://discord.com/developers/applications).

2. Click on `New Application`, give it a name and register it to yourself.

3. Read the TOS and Developer Policy and agree. Hit `Create`.

4. Select your new App and open the `OAuth2` Section. Reset your `Client Secret` and make sure to write it down in a safe place.

5. Now head to the OAuth2 section `URL Generator`. Select `bot` as the scope and add the necessary bot permissions*

6. Copy the link to your clipboard but don't open it yet.

7. Click on the section `Bot` and uncheck the switch called `Public Bot`

8. Scroll down to the switch called `Message Content Intent`. Check the switch. This will be necessary to read the messages in channels.

9. Now paste the link into your browser and add the bot to your discord server. If everything worked, the bot should now be on your discord server and work as expected.


*required permissions are: `bot, applications.commands, Read Messages/View Channels, Send Messages, Attach Files, Read Message History`

## Additional useful links

- API reference for interactions
  - [application command reference](https://discord.com/developers/docs/interactions/application-commands)
  - [general discord api reference](https://discord.com/developers/docs/reference)
