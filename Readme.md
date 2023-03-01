# game_chess

[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![desktop](https://img.shields.io/github/actions/workflow/status/Wandalen/game_chess/DesktopPush.yml?label=Desktop&logo=github)](https://github.com/Wandalen/game_chess/actions/workflows/DesktopPush.yml) [![web](https://img.shields.io/github/actions/workflow/status/Wandalen/game_chess/WebPush.yml?label=Web&logo=github)](https://github.com/Wandalen/game_chess/actions/workflows/WebPush.yml) [![beta](https://img.shields.io/github/actions/workflow/status/Wandalen/game_chess/Beta.yml?label=&branch=beta&logo=github)](https://github.com/Wandalen/game_chess/actions/workflows/Beta.yml)

## Desktop Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-make](https://github.com/sagiegurari/cargo-make)
- [xcb](https://xcb.freedesktop.org/)
- [libuv](https://github.com/libuv/libuv)
- [libalsa](https://www.alsa-project.org/wiki/Main_Page)

The project uses utility [cargo-make](https://github.com/sagiegurari/cargo-make). To install it run:

```
cargo install cargo-make
```

To build `Bevy` on Linux the libraries `libuv` and `libalsa` are required. To install it run:

```
sudo apt install libudev-dev libalsa-ocaml-dev
```

The project uses module [`Egui`](https://github.com/emilk/egui) and plugin [`bevy_egui`](https://github.com/mvlabat/bevy_egui). To compile the modules on Linux install `xcb`:

```
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

The project uses [`protobuff`](https://developers.google.com/protocol-buffers) to implement multiplayer. To install it:

- Linux:
```
sudo apt install protobuf-compiler
```
- MacOS:
```
brew install protobuf
```
- Pre-compiled binaries (any OS):
1) Load from [`here`](https://github.com/protocolbuffers/protobuf/releases/) for your system. File which you need is `protoc-<version>-<sys-name>.zip`
2) Unzip to some folder
3) Add the path to this folder to your system environment variable `Path`. The path should look like this: `<your_path>/protoc/bin`

## How to run on Desktop

To run desktop target execute:
```
cargo run
```

## How to develop

Before cloning the repository please make a [fork on Github](https://github.com/Wandalen/game_chess.git) to been able to open pull requests.
Please open pull request into the branch `alpha`.

## Platforms

Supported platforms:

- [Desktop](./doc/platform/Desktop.md) ( _default_ )
- [Web](./doc/platform/Web.md)

To run the project on default platform execute:

```
cargo run
```

## Frequently Asked Questions

Take a look at [FAQ](./doc/FAQ.md) page.

