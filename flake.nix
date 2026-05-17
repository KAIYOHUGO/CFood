{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
      ];
      systems = [ "x86_64-linux" ];
      perSystem =
        {
          config,
          pkgs,
          ...
        }:
        {
          devShells.default =
            with pkgs;
            mkShell {
              buildInputs = [
                llvm_18
              ];
              nativeBuildInputs = [
                rustc
                cargo
                rustfmt
                jre11_minimal
                just
              ];
            };
        };
      flake = {
      };
    };
}
