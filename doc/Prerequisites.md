## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) v1.57 or newer.
- [cargo-make](https://github.com/sagiegurari/cargo-make)
- [xcb](https://xcb.freedesktop.org/)
- [libuv](https://github.com/libuv/libuv)
- [libalsa](https://www.alsa-project.org/wiki/Main_Page)

The latest stable version of the Rust build toolchain is required by the project. To setup toolchain run:

```
rustup default stable
rustup update
```

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