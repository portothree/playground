{ pkgs ? import <nixpkgs> { }, shellHook ? ''
  EDITOR=vim
'' }:

pkgs.mkShell {
  buildInputs = with pkgs; [ postgresql ];
  inherit shellHook;
}
