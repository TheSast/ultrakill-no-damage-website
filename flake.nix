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
                # rust
                taplo-lsp
                cargo-edit
                (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) # rust-src, rust-analyzer
              ]
              ++ [
                # project
                cargo-generate # runtime dep of cargo-leptos
                cargo-leptos
                leptosfmt
                sass
              ]
              ++ [
                # lsp
                nodePackages.vscode-css-languageserver-bin
                prettierd
              ];
          };
      }
    );
}
