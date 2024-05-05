{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = pkgs.callPackage naersk {};
        sysdeps = [pkgs.portmidi];
      in {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = sysdeps;
        };
        devShell = pkgs.mkShell {
          packages = sysdeps ++ [pkgs.vmpk];
          buildInputs = with pkgs; [cargo rustc rustfmt rust-analyzer rustPackages.clippy];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    );
}
