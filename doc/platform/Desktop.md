## Desktop Prerequisites

- [Rust](https://www.rust-lang.org/) v1.57 or newer.
- [cargo-make](https://github.com/sagiegurari/cargo-make)
  Install with command: ```cargo install cargo-make```


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

## How to run on Desktop

To run desktop target execute:
```
cargo make desktop_run
```

To get list of all command related this target run:

```
cargo make --list-all-steps
```
