{
  description = "Minimal Rust Development Environment";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    andoriyu = {
      url = "github:andoriyu/flakes";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        fenix.follows = "fenix";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
    andoriyu,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      cwd = builtins.toString ./.;
      overlays = [ fenix.overlays.default ];
      pkgs = import nixpkgs {inherit system overlays;};
    in
      with pkgs; {
        formatter = nixpkgs.legacyPackages.${system}.alejandra;
        devShell = clangStdenv.mkDerivation rec {
          name = "rust";
          nativeBuildInputs = [
            (with fenix.packages.${system};
              combine [
                (stable.withComponents [
                  "cargo"
                  "clippy"
                  "rust-src"
                  "rustc"
                  "rustfmt"
                ])
                targets.wasm32-unknown-unknown.stable.rust-std
              ])
            bacon
            cargo-nextest
            cargo-outdated
            cmake
            gnumake
            pkg-config
            rust-analyzer-nightly
            sqlite
            zlib
          ];
          PROTOC = "${protobuf}/bin/protoc";
          PROTOC_INCLUDE = "${protobuf}/include";
        };
      });
}
