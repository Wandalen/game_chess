# game_chess
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![desktop](https://github.com/Wandalen/game_chess/actions/workflows/DesktopPush.yml/badge.svg)](https://github.com/Wandalen/game_chess/actions/workflows/DesktopPush.yml) [![web](https://github.com/Wandalen/game_chess/actions/workflows/WebPush.yml/badge.svg)](https://github.com/Wandalen/game_chess/actions/workflows/WebPush.yml) [![Beta](https://github.com/Wandalen/game_chess/actions/workflows/Beta.yml/badge.svg)](https://github.com/Wandalen/game_chess/actions/workflows/Beta.yml)

## How to use

The latest stable version of Rust build toolchain is required by the project. To setup toolchain run:

```
rustup default stable
rustup update
```

The project uses utility [cargo-make](https://github.com/sagiegurari/cargo-make). To install it run:

```
cargo install cargo-make
```

To build `Bevy` apps on a Linux distributive the libraries `libuv` and `libalsa` are required. To install it run:

```
sudo apt install libudev-dev libalsa-ocaml-dev
```

The project uses module [`Egui`](https://github.com/emilk/egui) and plugin [`bevy_egui`](https://github.com/mvlabat/bevy_egui). To compile the modules on Linux install `xcb`:

```
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

<!--
  the add instruction is considered correct because the result of testing
  https://github.com/Wandalen/game_chess/actions/runs/1618686028
  is ok
-->

## Platforms

Supported platforms:

- [Desktop](./doc/platform/Desktop.md) ( _default_ )
- [Web](./doc/platform/Web.md)

To run the project on default platform execute:

```
cargo make
```
