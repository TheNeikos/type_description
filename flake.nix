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
        wasm-bindgen-cli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "wasm-bindgen-cli";
          version = "0.2.83";

          src = pkgs.fetchCrate {
            inherit pname version;
            sha256 = "sha256-+PWxeRL5MkIfJtfN3/DjaDlqRgBgWZMa6dBt1Q+lpd0=";
          };

          cargoSha256 = "sha256-GwLeA6xLt7I+NzRaqjwVpt1pzRex1/snq30DPv4FR+g=";

          nativeBuildInputs = [ pkgs.pkg-config ];

          buildInputs = [ pkgs.openssl ];

          checkInputs = [ pkgs.nodejs ];

          # other tests require it to be ran in the wasm-bindgen monorepo
          cargoTestFlags = [ "--test=interface-types" ];

          meta = with pkgs.lib; {
            homepage = "https://rustwasm.github.io/docs/wasm-bindgen/";
            license = with licenses; [ asl20 /* or */ mit ];
            description = "Facilitating high-level interactions between wasm modules and JavaScript";
            maintainers = with maintainers; [ nitsky rizary ];
            mainProgram = "wasm-bindgen";
          };
        };

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

          buildPhaseCargoCommand = "trunk build online_description_generator/index.html --dist $out --release";

          postFixup = "rm -rf $out/target";

          pname = "type_description_website";

          nativeBuildInputs = [
            pkgs.trunk
            pkgs.binaryen
            pkgs.nodePackages.sass
            wasm-bindgen-cli
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
            wasm-bindgen-cli
            pkgs.binaryen
            pkgs.nodePackages.sass
          ];

          # Tell the trybuild crate to not use the wip folder
          TRYBUILD="overwrite";
        };
        devShells.default = devShells.type_description;
      }
    );
}
