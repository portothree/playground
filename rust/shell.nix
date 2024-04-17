{ pkgs ? import <nixpkgs> { }, shellHook ? "" }:

pkgs.mkShell {
  buildInputs = with pkgs; [ rustc cargo rustfmt ];
  inherit shellHook;
}
