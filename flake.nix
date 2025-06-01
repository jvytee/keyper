{
  description = "Minimalist authorization server";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs, fenix }:
    let
      eachSystem = nixpkgs.lib.genAttrs systems;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in {
      devShell = eachSystem (system:
        with import nixpkgs { inherit system; };
        mkShell {
          nativeBuildInputs = [
            fenix.packages.${system}.stable.toolchain
            yaml-language-server
          ];
        }
      );
    };
}
