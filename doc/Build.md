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
cargo make web_run_watching
```

This command doesn't perform crate installation checks to reduce total build time.


## Commands

To get list of commands use command `cargo make --list-all-steps`

```
Default
----------
default - Build debug of desktop target.

Desktop
----------
desktop_build - Build debug of desktop target.
desktop_build_release - Build release of desktop target.
desktop_rebuild - Rebuild debug of desktop target.
desktop_run - Run debug of desktop target
desktop_run_watching - Rerun debug desktop target on file change.

General
----------
doc - Generate full documentation
doc_dep_graph - Generate dependancy graph.
doc_ref - Generate reference
test - Run tests

Web
----------
web_build - Build debug of web target.
web_build_release - Build release of web target.
web_rebuild - Rebuild debug of web target.
web_run - Build and run web target.
web_run_watching - Rebuild debug of web target and run it.
web_server_run - Run HTTP server on port ${CARGO_MAKE_WEB_PORT}.
```


