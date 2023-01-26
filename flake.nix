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

        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;


        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        inherit (tomlInfo) pname version;
        src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;

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

        description_website = craneLib.mkCargoDerivation {
          inherit cargoArtifacts src version;

          CARGO_NET_OFFLINE = "true";
          TRUNK_STAGING_DIR = "/tmp/trunk-staging";
          XDG_CACHE_HOME = "/tmp/trunk-cache";
          RUST_LOG="trace";
          buildPhaseCargoCommand = "trunk -v build online_description_generator/index.html --dist $out --release";

          pname = "type_description_website";

          nativeBuildInputs = [
            pkgs.trunk
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
            pkgs.nodePackages.sass
          ];
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
        packages.description_website = description_website;
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
            pkgs.trunk
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
            pkgs.nodePackages.sass
          ];
        };
        devShells.default = devShells.type_description;
      }
    );
}
