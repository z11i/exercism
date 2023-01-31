{ nixpkgs ? import <nixpkgs> {} }:

let
  myPackages = with nixpkgs; [
    swiProlog
  ];

in

nixpkgs.stdenv.mkDerivation {
  name = "prolog-env";
  buildInputs = myPackages;
}
