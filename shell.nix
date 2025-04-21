{ fenix ? (import (builtins.fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { })
, pkgs ? (import <nixpkgs> {
  })
, system ? builtins.currentSystem
}:
let
  fenix' = fenix.packages.${system};
in
pkgs.callPackage
  ({ mkShell, lib, openssl, buildPackages, stdenv, windows, libgit2, pkg-config }: mkShell rec {
    depsBuildBuild = [
      pkg-config
      openssl
    ];
  LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
    nativeBuildInputs = [
      (fenix'.combine [
        (fenix'.complete.withComponents [
          "cargo"
          "rust-src"
          "clippy"
          "rustc"
        ])
        fenix'.rust-analyzer
        fenix'.latest.rustfmt
      ])
    ];
  })
{ }
