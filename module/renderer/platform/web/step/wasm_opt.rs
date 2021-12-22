//! ```cargo
//! [dependencies]
//! anyhow = "1.0"
//! binary-install = { version = "0.0.2" }
//! platforms = { version = "1.0.3" }
//! ```

use anyhow::{Context, Result};
use binary_install::Cache;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdout, Command, Stdio};


fn install_wasm_opt( target_path: impl AsRef<Path> ) -> Result<PathBuf>
{
  let cache = Cache::at( target_path.as_ref() );

  let url = format!
  (
    "https://github.com/WebAssembly/binaryen/releases/download/version_{version}/binaryen-version_{version}-{arch}-{os}.tar.gz",
    version = "103",
    arch = platforms::TARGET_ARCH,
    os = platforms::TARGET_OS,
  );

  #[cfg(target_os = "macos")]
  let binaries = &[ "wasm-opt", "libbinaryen" ];
  #[cfg(not(target_os = "macos"))]
  let binaries = &[ "wasm-opt" ];

  Ok
  (
    cache
    .download( true, "wasm-opt", binaries, &url )
    .map_err( |err| err.compat() )
    .with_context( || format!( "Could not download binaryen: {}", url ) )?
    .expect("Install is permitted.")
    .binary( "wasm-opt" )
    .map_err( |err| err.compat() )?
  )
}

fn main() -> ()
{
  let cwd_path = std::env::var( "CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY" ).unwrap();
  let cwd = std::path::Path::new( &cwd_path );
  let target_path = cwd.join( "../../../../bin" );
  let wasm_opt = install_wasm_opt( target_path ).unwrap();

  let mut args: Vec<String> = std::env::args().collect();

  let mut command = Command::new( &wasm_opt );

  command.args( &args[1..] );

  #[cfg(target_os = "macos")]
  {
    command.env( "DYLD_LIBRARY_PATH", wasm_opt.parent().unwrap() );
  }

  let status = command.status().expect( "Command wasm-opt failed to start" );

  std::process::exit( status.code().unwrap() );
}