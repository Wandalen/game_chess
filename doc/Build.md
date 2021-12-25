# How to build

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


