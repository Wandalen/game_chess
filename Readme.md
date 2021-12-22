# game_chess
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![desktop](https://github.com/obox-systems/game_chess/actions/workflows/DesktopPush.yml/badge.svg)](https://github.com/obox-systems/game_chess/actions/workflows/DesktopPush.yml) [![web](https://github.com/obox-systems/game_chess/actions/workflows/WebPush.yml/badge.svg)](https://github.com/obox-systems/game_chess/actions/workflows/WebPush.yml)

## Build tool

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) task runner. It's required to build the project. To install it run:

```
cargo install cargo-make
```

## Platforms

Supported platforms:

- [Desktop](./doc/platform/Desktop.md) ( _default_ )
- [Web](./doc/platform/Web.md)

To run the project on default platform execute:

```
cargo make
```
