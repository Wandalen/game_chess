# game_chess
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![boardtop](https://github.com/Wandalen/game_chess/actions/workflows/boardtopPush.yml/badge.svg)](https://github.com/Wandalen/game_chess/actions/workflows/boardtopPush.yml) [![web](https://github.com/Wandalen/game_chess/actions/workflows/WebPush.yml/badge.svg)](https://github.com/Wandalen/game_chess/actions/workflows/WebPush.yml)

## General prerequisites

```
cargo install cargo-make
```

## How to run on Desktop

```
cargo make boardtop_run
```

## How to run on Web

Run web target:

```
cargo make web_run
```

To speedup incremental builds use:

```
cargo make web_rerun_watching
```

This command doesn't perform crate installation checks to reduce total build time.


## Commands

To get list of commands use command `cargo make --list-all-steps`

```
Prerequisites
----------
web_install_dependencies - Install web dependencies

Desktop
----------
boardtop_build - Build boardtop target.
boardtop_run - Run boardtop target
boardtop_run_watching - Run boardtop target. Rebuilds app on change

Web
----------
web_build - Build web target. Rebuilds on change.
web_rebuild - Build web target. Rebuilds on change.
web_rerun - Run web target. Rebuilds app on change
web_run - Run web target. Rebuilds app on change
```


