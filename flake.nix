{
  description = "The type_description rust library flake";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.05";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
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
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;


        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        inherit (tomlInfo) pname version;
        src = pkgs.nix-gitignore.gitignoreSource [] ./.;

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
        };

        type_description = craneLib.buildPackage {
          inherit cargoArtifacts src version;
        };

        book = craneLib.mkCargoDerivation {
          inherit cargoArtifacts src version;
          cargoVendorDir = null;

          buildPhaseCargoCommand = "mdbook build doc_guide -d $out";
          checkPhaseCargoCommand = "mdbook test doc_guide -L target/debug/deps";

          postFixup = "rm -rf $out/target";

          pname = "type_description_book";

          nativeBuildInputs = [ pkgs.mdbook ];
        };
      in
      rec {
        checks = {
          inherit type_description book;

          type_description-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src;
            cargoClippyExtraArgs = "-- --deny warnings";
          };

          type_description-fmt = craneLib.cargoFmt {
            inherit src;
          };
        };

        packages.book = book;
        packages.type_description = type_description;
        packages.default = packages.type_description;

        devShells.type_description = pkgs.mkShell {
          nativeBuildInputs = [
            rustTarget

            pkgs.cargo-msrv
            pkgs.cargo-edit
            pkgs.cargo-deny
            pkgs.cargo-expand
            pkgs.cargo-bloat

            pkgs.mdbook
          ];
        };
        devShells.default = devShells.type_description;
      }
    );
}
