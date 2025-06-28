{
  description = "A simple CLI to fetch stock ticker prices";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;
        src = ./.;
        buildInputs = with pkgs; [ ];
      in
      {
        packages.default = craneLib.buildPackage {
          inherit src;
          buildInputs = with pkgs; [ ];
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/stock-ticker";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust
            cargo
            cargo-watch
          ];
        };
      });
}
