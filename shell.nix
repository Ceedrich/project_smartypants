{ pkgs ? import <nixpkgs> {} }:

let
  libraries = with pkgs; [
    xorg.libX11
    xorg.libXext
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXrender
    xorg.libXfixes
    xorg.libXft
    fontconfig
    pango
    cairo
    gobject-introspection
  ];
in
pkgs.mkShell {
  buildInputs = libraries;

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH"
  '';
}
