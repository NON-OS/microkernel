# Every tool the build runs, at one version, held here or in flake.lock.
#
# Rust comes from rust-toolchain.toml through rust-overlay, so rustup users and
# the flake read the same pin and cannot drift. zig 0.16.0, cmake 4.4.3 and
# Go 1.26.8 are the releases the Linux userland was built and checked with;
# nixpkgs carries each at that version, and the asserts below fail the
# evaluation if a nixpkgs update moves one.
{ pkgs, lib, self }:
let
  rust = pkgs.rust-bin.fromRustupToolchainFile (self + "/rust-toolchain.toml");
  # The same toolchain with the UEFI target's prebuilt core, which the
  # bootloader builds against (nonos-bootloader/rust-toolchain.toml names it).
  rustUefi = rust.override { targets = [ "x86_64-unknown-uefi" ]; };
  # The shell's toolchain: the UEFI target, wasm32 for the verifier the release
  # lane ships with the attestation surface (stark-attest), and the static musl
  # target so the make lane builds the Linux userland's Rust tools (ripgrep, fd)
  # in `nix develop` the same way the nix lane builds them with rustMusl.
  rustShell = rust.override { targets = [ "x86_64-unknown-uefi" "wasm32-unknown-unknown" "x86_64-unknown-linux-musl" ]; };
  # The same toolchain with the static musl target, for the Rust programs in
  # the Linux userland (ripgrep, fd).
  rustMusl = rust.override { targets = [ "x86_64-unknown-linux-musl" ]; };
  zig = pkgs.zig_0_16;
  cmake = pkgs.cmake;
  # The Go the Linux userland's Go programs (gojq) are built with.
  go = pkgs.go_1_26;
  # The C compiler for every C file built into a NONOS binary (the kernel's
  # pqclean and assembly, QuickJS and minimp3 in the capsules): one clang,
  # which targets any triple, so no host compiler reaches an artifact.
  llvm = pkgs.llvmPackages_21;
  # The software TPM the live TPM proofs run against, on a released libtpms.
  # nixpkgs carries an unreleased snapshot, which no longer answers the first
  # quote after a startup with TPM_RC_RETRY as released reference code does,
  # and the proofs hold the kernel to that answer.
  libtpms = pkgs.libtpms.overrideAttrs (_: {
    version = "0.9.6";
    # The snapshot's patches are for the snapshot.
    patches = [ ];
    # gcc 16 warns where gcc of the release's day did not, and the release
    # builds with -Werror. The warning is gcc's; clang, on macOS, knows no
    # such option and refuses it.
    env.NIX_CFLAGS_COMPILE = lib.optionalString pkgs.stdenv.cc.isGNU "-Wno-error=discarded-qualifiers";
    src = pkgs.fetchFromGitHub {
      owner = "stefanberger";
      repo = "libtpms";
      rev = "v0.9.6";
      hash = "sha256-I2TYuOLwgEm6ofF2onWI7j2yu9wpXxNt7lJePSpF9VM=";
    };
  });
  # The released pair is what the live TPM proofs run against, and they run on
  # Linux only (tpm2-tools is Linux-only). On macOS the shell and the QEMU
  # boot take nixpkgs' own swtpm, which builds there; the release does not
  # under the newer clang.
  swtpm = if !pkgs.stdenv.hostPlatform.isLinux then pkgs.swtpm else (pkgs.swtpm.override { inherit libtpms; }).overrideAttrs (old: {
    version = "0.9.0";
    src = pkgs.fetchFromGitHub {
      owner = "stefanberger";
      repo = "swtpm";
      rev = "v0.9.0";
      hash = "sha256-IeFrS67qStklaTgM0d3F8Xt8upm2kEawT0ZPFD7JKnk=";
    };
    patches = [ ];
    # The proofs talk to it over TCP; the character device interface wants a
    # FUSE the release predates.
    configureFlags = builtins.filter (f: f != "--with-cuse") old.configureFlags ++ [ "--without-cuse" ];
    # What nixpkgs does to the snapshot, for the release's sources: openssl
    # is found on PATH, which every user of this swtpm puts there.
    postPatch = ''
      patchShebangs tests/*
      substituteInPlace samples/Makefile.am --replace-fail 'install-data-local:' 'do-not-execute:'
    '';
  });
in
assert lib.assertMsg (zig.version == "0.16.0") "zig is ${zig.version}, the pin is 0.16.0";
assert lib.assertMsg (cmake.version == "4.4.3") "cmake is ${cmake.version}, the pin is 4.4.3";
assert lib.assertMsg (go.version == "1.26.8") "go is ${go.version}, the pin is 1.26.8";
assert lib.assertMsg (lib.versions.major llvm.clang-unwrapped.version == "21") "clang is ${llvm.clang-unwrapped.version}, the pin is 21";
{
  inherit rust rustUefi rustShell rustMusl zig cmake go llvm swtpm;
  python = pkgs.python312;
  # The same interpreter with the one package the key tools import
  # (tools/nonos-seal, tools/nonos-policy-approve, tools/nonos-key-ceremony):
  # what the shell, the apps and the checks run Python scripts with.
  pythonTools = pkgs.python312.withPackages (p: [ p.cryptography ]);
}
