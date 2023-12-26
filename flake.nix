{
  description = "uknd";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default =
          pkgs.mkShell
          {
            packages = with pkgs;
              [
                cargo
                clippy
                rustfmt
                (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) # rust-src, rust-analyzer
              ]
              ++ [
                cargo-generate # runtime dep of cargo-leptos
                cargo-leptos
                leptosfmt
                sass
              ]
              ++ [
                nodePackages.vscode-css-languageserver-bin
                prettierd
              ];
          };
      }
    );
}
