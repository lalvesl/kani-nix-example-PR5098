{
  description = "Using Kani from Nix, with a development shell per Kani version";

  inputs = {
    # Kani's flake: an overlay with every recorded Kani release.
    kani.url = "github:lalvesl/kani/nix-flake";
    nixpkgs.follows = "kani/nixpkgs";
  };

  outputs =
    { kani, nixpkgs, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ kani.overlays.default ];
            }
          )
        );
    in
    {
      devShells = forAllSystems (
        pkgs:
        let
          # `release` brings Kani and the exact nightly toolchain it was built with.
          kaniShell =
            release:
            pkgs.mkShell {
              packages = [
                release.kani
                # The project itself can use any toolchain: Kani compiles it with its own.
                pkgs.rust-bin.stable.latest.default
              ];
              # `cargo` looks for subcommands in `$CARGO_HOME/bin` before `PATH`, unless that
              # directory is on `PATH`. Put it last, so this shell's `cargo-kani` wins over one
              # installed with `cargo install kani-verifier`.
              shellHook = ''
                export PATH="$PATH:''${CARGO_HOME:-$HOME/.cargo}/bin"
              '';
            };
        in
        rec {
          kani-latest = kaniShell pkgs.kaniReleases.latest;
          kani-0_67 = kaniShell pkgs.kaniReleases."0.67.0";
          default = kani-latest;
        }
      );
    };
}
