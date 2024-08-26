{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        devDependencies = with pkgs; [
          pkg-config
          cargo
          rustc
          rust-analyzer
          rustfmt
          clippy
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = devDependencies;
        };
      });
}
