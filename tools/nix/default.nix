# The flake's body, one system at a time. Each file here does one job; this one
# wires them together.
{ self, nixpkgs, rust-overlay, starks, system }:
let
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rust-overlay.overlays.default ];
  };
  inherit (pkgs) lib;

  pins = import ./pins.nix { inherit pkgs lib self; };
  src = import ./src.nix { inherit lib; };
  cargo = import ./vendor.nix { inherit pkgs lib self starks; rust = pins.rust; };
  rustBuild = import ./rustbuild.nix { inherit pkgs lib pins cargo; };
  userland = import ./userland.nix { inherit pkgs lib pins src cargo; };
  periodic = import ./periodic.nix { inherit pins rustBuild starks; };
  capsules = import ./capsules.nix {
    inherit pkgs lib pins rustBuild src periodic;
    linuxUserland = userland.all;
    busybox = userland.busybox;
    packageMirror = config.file.linux_packages or "";
  };
  image = import ./image.nix { inherit pkgs lib pins rustBuild src capsules self; };
  config = import ./config.nix { inherit lib; root = src.root; };

  # Every lock file a build in this flake reads, for the bill of materials.
  lockFiles =
    let
      k = src.root;
    in
    [ (k + "/Cargo.lock") (k + "/nonos-bootloader/Cargo.lock") cargo.rustSrcLock ./locks/ripgrep-14.1.1.Cargo.lock ./locks/ripgrep-15.2.0.Cargo.lock ./locks/fd-find-10.5.0.Cargo.lock ]
    ++ map (e: k + "/${e.dir}/Cargo.lock") (builtins.filter (e: e.prebuilt == "") capsules.catalogue)
    ++ map (t: k + "/userland/upstream-src/${t}/Cargo.lock") [ "sd-1.0.0" "tokio-smoke" "grex" "dotenv-linter" "pastel" "jsonxf" "tokei" "huniq" "csview" ]
    ++ map (d: k + "/${d}/Cargo.lock") [ "toolchain/nonos-rt" "nonos-sign" "nonos-stark-enroll" "nonos-bootloader/tools/embed-trailer" "nonos-bootloader/tools/sign-kernel" "nonos-mk" "tools/nonos-pack" "nonos-verify" "nonos-proof-verify" "nonos-proof-verify/wasm" ];

  sbom = import ./sbom.nix { inherit pkgs lib pins userland lockFiles self; };
  artifacts = import ./artifacts.nix { inherit pkgs lib pins capsules userland image sbom; };

  # One build, configured: nonos.toml as written, and each profile with the
  # rest of nonos.toml unchanged, for the lanes that build more than one.
  configured = config.resolve config.file;
  profile = p: config.resolve (config.file // { profile = p; });

  hostTools = pkgs.symlinkJoin {
    name = "nonos-host-tools";
    paths = builtins.attrValues image.hostTools;
  };

  shell = import ./shell.nix { inherit pkgs lib pins userland capsules; };
  checks = import ./checks.nix {
    inherit pkgs lib pins cargo rustBuild src capsules config image self starks periodic;
    busybox = userland.busybox;
  };
  apps = import ./apps.nix { inherit pkgs lib hostTools shell; };
in
{
  packages = {
    default = artifacts configured;
  }
  // lib.genAttrs (builtins.attrNames config.profiles) (p: artifacts (profile p))
  # Each profile's development twin, for a test boot without the prover.
  // lib.listToAttrs (map (p: lib.nameValuePair "${p}-dev" (artifacts (config.resolve (config.file // { profile = p; dev = true; }))))
    (builtins.filter (p: p != "dev") (builtins.attrNames config.profiles)))
  // {

    kernel = image.kernel configured;
    bootloader = image.bootloader configured;
    capsules = pkgs.linkFarm "nonos-capsules" (lib.mapAttrsToList (n: v: { name = n; path = v; }) capsules.bySlug);
    # The development images' set (capsules.nix, bySlugDev): what the seal
    # builds for an image that is not a release, and refuses for one.
    capsules-dev = pkgs.linkFarm "nonos-capsules-dev" (lib.mapAttrsToList (n: v: { name = n; path = v; }) capsules.bySlugDev);
    linux-userland = userland.all;
    busybox = userland.busybox;
    # The STARK verifier for a web page, nox_verify_wasm from NON-OS/STARKs at
    # the commit flake.lock pins: the gates' own verifier, so the verify page,
    # the browser and the boot check the same way.
    verifier-wasm = rustBuild {
      name = "nox-verify-wasm";
      src = starks;
      lockFiles = [ (starks + "/Cargo.lock") ];
      rust = pins.rustShell;
      cc = false;
      script = ''
        cargo build --frozen --release -p nox_verify_wasm --target wasm32-unknown-unknown
        mkdir -p $out
        cp target/wasm32-unknown-unknown/release/nox_verify_wasm.wasm $out/verifier.wasm
        cp nox_verify_wasm/js/*.mjs $out/
      '';
    };
    # The proof verifier's page: nonos-proof-verify compiled to wasm, beside
    # the page that runs it. The module runs the host tool's own command line,
    # so the page and `nonos-proof-verify` answer alike (web/smoke.mjs).
    proof-verify-web = rustBuild {
      name = "nonos-proof-verify-web";
      src = src.crate "nonos-proof-verify/wasm" [ ];
      lockFiles = [ (src.root + "/nonos-proof-verify/wasm/Cargo.lock") ];
      rust = pins.rustShell;
      cc = false;
      script = ''
        cd nonos-proof-verify/wasm
        cargo build --frozen --release --target wasm32-unknown-unknown
        mkdir -p $out
        cp target/wasm32-unknown-unknown/release/nonos_proof_verify_wasm.wasm $out/
        cp ../web/index.html ../web/app.js ../web/npv.mjs $out/
      '';
    };
    # The periodic cache nonos.shield embeds, built and checked by nox_bench
    # at the same commit.
    periodic-cache = periodic;
    profiles = pkgs.writeText "nonos-profiles.txt" config.describe;
    host-tools = hostTools;
    inherit sbom;

    toolchain = pins.rust;
    rust-nonos-std = capsules.rustStd;
    nonos-rt = capsules.rt;
  }
  // lib.mapAttrs' (n: v: lib.nameValuePair "capsule-${n}" v) capsules.bySlug
  // lib.mapAttrs' (n: v: lib.nameValuePair "linux-${n}" v) userland.programs
  // lib.mapAttrs' (n: v: lib.nameValuePair "upstream-${n}" v) capsules.upstream
  // lib.mapAttrs' (n: v: lib.nameValuePair "host-${n}" v) image.hostTools;

  inherit checks apps;
  devShells.default = shell;
  formatter = pkgs.nixfmt;
}
