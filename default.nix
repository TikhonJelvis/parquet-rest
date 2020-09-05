{ sources ? import nix/sources.nix {} }:

let
  pkgs = import sources.nixpkgs {};
  rust = import ./nix/rust.nix { inherit sources; };

  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
in naersk.buildPackage {
  # Filter target directory created by Cargo
  src = builtins.filterSource
    (path: type:
      type != "directory" || builtins.baseNameOf path != "target")
    ./.;

  # Rmove nix store references for a smaller output package.
  remapPathPrefix = true;
}
