{
  description = "Dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.protobuf
            pkgs.openssl
            pkgs.pkg-config
            pkgs.rustc
            pkgs.cargo
            pkgs.protobuf
            pkgs.rustfmt
          ];

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          shellHook = ''
            echo "Dev shell"
            export PATH=$PATH:${pkgs.protobuf}/bin
          '';
        };
      });
}
