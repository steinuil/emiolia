{ pkgs, ... }:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.stdenv.mkDerivation {
  name = "${manifest.name}-dev";

  # Compile time dependencies
  nativeBuildInputs = with pkgs; [
    # Nix
    nixd
    statix
    deadnix
    alejandra

    # Rust
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-watch

    # Other compile time dependencies
    openssl

    # libsoup
    libsoup_3
    gnutls
    glib-networking

    # PDF
    gtk4-layer-shell
    poppler

    # Gnome related
    gtk4
    meson
    ninja
    # parted
    gettext
    appstream
    pkg-config
    gdk-pixbuf
    libadwaita
    gnome-desktop
    wrapGAppsHook4
    desktop-file-utils
    gobject-introspection
    gsettings-desktop-schemas
    rustPlatform.bindgenHook
  ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  GSETTINGS_SCHEMA_DIR = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/gsettings-desktop-schemas-49.1/glib-2.0/schemas:${pkgs.gtk4}/share/gsettings-schemas/gtk4-4.22.0/glib-2.0/schemas";
}
