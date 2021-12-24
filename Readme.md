# game_chess
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![desktop](https://github.com/Wandalen/game_chess/actions/workflows/DesktopPush.yml/badge.svg)](https://github.com/Wandalen/game_chess/actions/workflows/DesktopPush.yml) [![web](https://github.com/Wandalen/game_chess/actions/workflows/WebPush.yml/badge.svg)](https://github.com/Wandalen/game_chess/actions/workflows/WebPush.yml)

## Build tool

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) task runner. It's required to build the project. To install it run:

```
cargo install cargo-make
```

The project uses GUI library [`Egui`](https://github.com/emilk/egui) and plugin [`bevy_egui`](https://github.com/mvlabat/bevy_egui). To compile the libraries on GNU/Linux distributives `xcb` libraries is required. To install it run:

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
