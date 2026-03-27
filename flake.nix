{
  description = "Replica";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: {
              gtk4 = prev.gtk4.overrideAttrs (prevAttrs: {
                version = "4.22.0";

                src = pkgs.fetchurl {
                  url = "mirror://gnome/sources/gtk/4.22/gtk-4.22.0.tar.xz";
                  hash = "sha256-7TkyRp09iR31NFHyzSg4CvdnP3NqpEO7Pdp620CZTx4=";
                };

                nativeBuildInputs = prevAttrs.nativeBuildInputs ++ [
                  prev.shared-mime-info
                ];
              });

              libadwaita = prev.libadwaita.overrideAttrs (finalAttrs: {
                version = "1.9.0";

                src = pkgs.fetchFromGitLab {
                  domain = "gitlab.gnome.org";
                  owner = "GNOME";
                  repo = "libadwaita";
                  tag = "1.9.0";
                  hash = "sha256-JAKP8CjLCKGZvHoB26ih/J3xAru4wiVf/ObG0L8r4pY=";
                };
              });
            })
          ];
        };
      in
      {
        formatter = pkgs.alejandra;

        devShells.default = import ./shell.nix { inherit pkgs; };

        # packages.default = pkgs.callPackage ./. {inherit pkgs;};
      }
    );
}
