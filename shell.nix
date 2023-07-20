with import <nixpkgs> { };

let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };

  rust = (nixpkgs.rustChannelOf {
    date = "2023-03-03";
    channel = "nightly";
  }).rust;

in mkShell {
  name = "nightly-rust";

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [

    (rust.override { targets = [ "wasm32-unknown-unknown" ]; })

    # Rust wasm builds
    wasm-pack
    openssl

    # for LLVM target
    llvmPackages_15.clangUseLLVM
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
