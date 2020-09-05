{ sources ? import ./sources.nix
, channel ? "nightly"
, date ? "2020-09-04"
, targets ? []
}:
let
  pkgs = import sources.nixpkgs {
    overlays = [
      (import sources.mozilla-overlay)
    ];
  };
in
pkgs.rustChannelOfTargets channel date targets
