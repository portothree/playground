{
  description = "rust";
  inputs = {
    nixpkgs = { url = "github:NixOS/nixpkgs/nixos-unstable"; };
    flake-utils = { url = "github:numtide/flake-utils"; };
    pre-commit-hooks = { url = "github:cachix/pre-commit-hooks.nix"; };
  };
  outputs = { self, nixpkgs, flake-utils, pre-commit-hooks }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        src = ./.;
      in {
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            inherit src;
            hooks = {
              nixfmt.enable = true;
              shfmt.enable = true;
            };
          };
        };
        devShell = import ./shell.nix {
          inherit pkgs;
          inherit (self.checks.${system}.pre-commit-check) shellHook;
        };
      });
}
