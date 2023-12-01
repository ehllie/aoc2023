{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.parts.lib.mkFlake { inherit inputs; } {
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    imports = [
      inputs.parts.flakeModules.easyOverlay
      inputs.nci.flakeModule
      ./crates.nix
    ];

    perSystem = { config, ... }:
      let crateOutputs = config.nci.outputs.aoc2023; in
      {
        overlayAttrs.aoc2023 = config.packages.default;
        packages.default = crateOutputs.packages.release;
        devShells.default = crateOutputs.devShell.overrideAttrs (_: {
          shellHook = ''
            PATH=$PATH:$PWD/target/debug
          '';
        });
      };
  };
}
