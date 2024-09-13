{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            (final: prev:
              let
                toolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
                  ./rust-toolchain.toml;
              in {
                rustPlatform = prev.makeRustPlatform {
                  cargo = toolchain;
                  rustc = toolchain;
                };
              })
          ];
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
          ./rust-toolchain.toml;

        nativeBuildInputs = with pkgs; [ ];

        buildInputs = with pkgs; [
          nixfmt
          curl
          docker
          git

          rustToolchain
          sccache

          cargo-expand
        ];
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in with pkgs; {
        packages = {
          sway-workspace-watcher = pkgs.rustPlatform.buildRustPackage rec {
            pname = manifest.name;
            version = manifest.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
          };
          default = self.packages.${system}.sway-workspace-watcher;
        };

        devShells.default = mkShell {
          inherit buildInputs nativeBuildInputs;

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
        };
      });
}
