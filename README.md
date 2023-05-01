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

### building

just use `cargo` for building:

```shell
cargo build --release && cargo run --release
```

### using the chatbot

1. (it is based on [Desinc's idea](https://github.com/DeSinc/SallyBot#using-oobabooga-text-generation-webui-run-on-the-gpu----a-little-more-involved-but-still-easy-if-youre-lucky) so it works the same way)
use the one-click installer for your specific operating system from the [github repo](https://github.com/oobabooga/text-generation-webui#installation)

2. run the setup command for your specific operating system, for windows:
`.\start-windows.bat`

3. Follow the instructions in the terminal

4. choose your ML model: i just used `facebook/opt-2.7b`

5. install xformers by running the `cmd_windows.bat` script and running `pip install xformers==0.0.18`

6. change the start-up arguments in `webui.py` by changing

```python
run_cmd("python server.py --auto-devices --chat --model-menu")
```

to

```python
run_cmd("python server.py --model ozcur_alpaca-native-4bit --wbits 4 --groupsize 128 --extensions api --notebook --listen-port 7862 --xformers")
```

7. start the program and have fun :D

### dependencies

- [3d-ascii-viewer](https://github.com/autopawn/3d-ascii-viewer-c)
  - some `.obj` files you want to render
- [oobabooga/text-generation-webui](https://github.com/oobabooga/text-generation-webui)
  - xformers 0.0.18
