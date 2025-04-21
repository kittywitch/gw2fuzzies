{
  description = "gw2fuzzies, eventually a fuzzer for builds";
  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, flake-utils, crane, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        # fuzzies devShell
        shellToolchain = with fenix.packages.${system};
          combine [
            complete
            rust-analyzer
          ];

        shellCraneLib = (crane.mkLib pkgs).overrideToolchain (p: shellToolchain);

        fuzziesShell = import ./shell.nix {
          inherit fenix pkgs system;
        };
      in
      rec {
        inherit pkgs;
        devShells.default = fuzziesShell;
      });
}

