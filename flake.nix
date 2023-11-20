{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = { self, nixpkgs, crane, rust-overlay, flake-utils, treefmt-nix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };


        rust = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "rust-analyzer" "clippy" ];
          targets = [ "aarch64-unknown-none" ];
        };

        riscv-toolchain = import nixpkgs {
          localSystem = "${system}";
          crossSystem = {
            config = "riscv64-unknown-linux-gnu";
          };
        };

        aarch64-toolchain = import nixpkgs {
          localSystem = "${system}";
          crossSystem = {
            config = "aarch64-unknown-none";
          };
        };

        haskell = (pkgs.haskellPackages.ghcWithPackages (ps: [
          ps.shake
          ps.haskell-language-server
          ps.ormolu
          ps.stack
        ]));
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ 
            rust 
            aarch64-toolchain.gcc 
          ];
        };
        formatter = treefmt-nix.lib.mkWrapper
          nixpkgs.legacyPackages.${system}
          {
            projectRootFile = "flake.nix";
            programs.nixpkgs-fmt.enable = true;
            programs.rustfmt.enable = true;
            programs.ormolu.enable = true;
          };
      });
}
