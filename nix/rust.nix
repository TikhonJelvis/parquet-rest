{ sources ? import ./sources.nix
, channel ? "stable"
, date ? "2020-08-27"
, targets ? []
}:
let
  pkgs = import sources.nixpkgs {
    overlays = [
      (import sources.mozilla-overlay)
    ];
  };
  rustChannel = pkgs.rustChannelOfTargets channel date targets;
in
rustChannel.override {
  extensions = [ "rust-src" "rustfmt-preview" ];
}
