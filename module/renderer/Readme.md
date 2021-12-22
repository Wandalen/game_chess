# Rust Game Template
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![ios](https://github.com/obox-systems/game_chess/actions/workflows/iOS.yml/badge.svg)](https://github.com/obox-systems/game_chess/actions/workflows/iOS.yml) [![android](https://github.com/obox-systems/game_chess/actions/workflows/Android.yml/badge.svg)](https://github.com/obox-systems/game_chess/actions/workflows/Android.yml) [![desktop](https://github.com/obox-systems/game_chess/actions/workflows/Desktop.yml/badge.svg)](https://github.com/obox-systems/game_chess/actions/workflows/Desktop.yml) [![web](https://github.com/obox-systems/game_chess/actions/workflows/Web.yml/badge.svg)](https://github.com/obox-systems/game_chess/actions/workflows/Web.yml)

Neutral cross-platform Rust game template.

## General prerequisites

```
cargo install cargo-make
```

## How to run on Desktop

```
cargo make desktop_run
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
desktop_build - Build desktop target.
desktop_run - Run desktop target
desktop_run_watching - Run desktop target. Rebuilds app on change

Web
----------
web_build - Build web target. Rebuilds on change.
web_rebuild - Build web target. Rebuilds on change.
web_rerun - Run web target. Rebuilds app on change
web_run - Run web target. Rebuilds app on change
```


