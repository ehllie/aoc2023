{
  perSystem = { pkgs, lib, ... }:
    let
      inherit (pkgs) stdenv pkg-config darwin openssl;
    in
    {
      nci = {
        toolchainConfig = ./rust-toolchain.toml;
        projects.aoc2023 = rec {
          path = ./.;
          depsDrvConfig = {
            mkDerivation = {
              nativeBuildInputs = lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.SystemConfiguration
              ] ++ lib.optionals stdenv.isLinux [
                pkg-config
                openssl
              ];
            };
          };
          drvConfig = depsDrvConfig;
        };
      };
    };
}
