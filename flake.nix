{
  # NONOS, built one way.
  #
  #   nix develop         the pinned toolchain, on Linux, macOS or WSL2
  #   nix build           the reproducible artifacts (kernel, capsules, Linux
  #                       userland, bootloader) for the build in nonos.toml
  #   nix flake check     every proof crate and static check; nothing is green
  #                       until they are
  #   nix run .#seal      ek's step: enroll, sign and pack, with ek's keys
  #   nix run .#qemu      boot a sealed image
  #
  # The flake stops at the reproducible artifacts. Enrollment draws fresh
  # randomness and signing needs keys the flake never sees, so the sealed image
  # is not bit identical between two seals, by design. tools/nix/README.md says
  # where that line runs and why.
  description = "NONOS: capability microkernel, RAM-resident, STARK-attested";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # The STARK prover and verifier the kernel, the loader and the enroll
    # tool are built with, pinned here and nowhere else. Every Cargo.lock
    # names this commit (tools/nonos-starks-sync writes it, flake check holds
    # them to it), and starks-bump.yml moves it when STARKs main moves, in a
    # pull request ek merges, because it is the trust path.
    starks = {
      url = "github:NON-OS/STARKs/main";
      flake = false;
    };
  };

  outputs =
    { self, nixpkgs, rust-overlay, starks }:
    let
      # nixpkgs no longer builds for Intel Macs; Apple silicon is the macOS host.
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      each = f: nixpkgs.lib.genAttrs systems (system: f (import ./tools/nix {
        inherit self nixpkgs rust-overlay starks system;
      }));
    in
    {
      packages = each (n: n.packages);
      checks = each (n: n.checks);
      apps = each (n: n.apps);
      devShells = each (n: n.devShells);
      formatter = each (n: n.formatter);
    };
}
