{
  description = "A command line tool for archiving MacOS Cocoa objects.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlay = rust-overlay.overlays.default;
        pkgs = import nixpkgs { inherit system overlay; };

        rustToolchain = pkgs.rust-bin.stable.latest.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
            "rust-analyzer"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
          ];
        };

        formatter = pkgs.nixfmt;
      }
    );
}
