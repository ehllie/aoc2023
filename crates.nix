{
  perSystem = { pkgs, lib, ... }: {
    nci = {
      toolchainConfig = ./rust-toolchain.toml;
      projects.aoc2023 = rec {
        path = ./.;
        depsDrvConfig = {
          mkDerivation = {
            nativeBuildInputs = lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.apple_sdk.frameworks.SystemConfiguration ];
          };
        };
        drvConfig = depsDrvConfig;
      };
    };
  };
}
