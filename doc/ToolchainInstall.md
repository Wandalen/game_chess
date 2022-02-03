# How to install Rust toolchain

### From official site

Visit the official page of Rust language and proceed to [installation page](https://www.rust-lang.org/tools/install).

### Linux and MocOS

Execute command in shell

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

Download x64 installation file [using the link](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) and install as regular Windows app.


### Check installation

Run command

```
rustup -V
```

Output should looks like:

```
rustup 1.24.3 (ce5817a94 2021-05-31)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.58.1 (db9d1b20b 2022-01-20)
```

