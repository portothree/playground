{
  description = "rust-cli";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, utils, crate2nix, ... }:
    let name = "rust-cli";
    in utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
          generatedCargoNix;

        project = pkgs.callPackage (generatedCargoNix {
          inherit name;
          src = ./.;
        }) {
          # Individual crate overrides go here
          # Example: https://github.com/balsoft/simple-osd-daemons/blob/6f85144934c0c1382c7a4d3a2bbb80106776e270/flake.nix#L28-L50
          defaultCrateOverrides = pkgs.defaultCrateOverrides // {
            ${name} = oldAttrs:
              {
                inherit buildInputs nativeBuildInputs;
              } // buildEnvVars;
          };
        };

        # Configuration for the non-Rust dependencies
        buildInputs = with pkgs; [ openssl.dev ];
        nativeBuildInputs = with pkgs; [ rustc cargo pkg-config nixpkgs-fmt ];
        buildEnvVars = {
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in rec {
        packages.${name} = project.rootCrate.build;
        defaultPackage = packages.${name};
        apps.${name} = utils.lib.mkApp {
          inherit name;
          drv = packages.${name};
        };
        defaultApp = apps.${name};
        devShell = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          RUST_SRC_PATH =
            "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        } // buildEnvVars;
      });
}
