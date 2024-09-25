{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [ postgresql ];
  shellHook = ''
    export PGHOST=localhost
    export PGUSER=postgres
    export PGPASSWORD=acuriousmoon
    export PGPORT=5436
  '';
}
