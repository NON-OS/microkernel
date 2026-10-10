# `nix develop`: the exact toolchain the flake builds with, plus what booting,
# sealing and checking need. Nothing here comes from the host, so a shell on
# Arch, Debian, macOS or WSL2 is the same shell.
{ pkgs, lib, pins, userland, capsules }:
let
  # Secure Boot signing and the TPM's command-line tools exist only for Linux;
  # on macOS the seal signs with osslsigncode and the TPM proofs run in CI.
  linuxOnly = lib.optionals pkgs.stdenv.hostPlatform.isLinux [ pkgs.sbsigntool pkgs.tpm2-tools ];
  # QEMU's own build of the UEFI firmware, the same on every host.
  firmware = "${pkgs.qemu}/share/qemu";
  # Every tool, as the packages themselves. The apps put these on PATH
  # (tools/nix/apps.nix): read back off the shell, mkShell's list holds each
  # package's dev output, whose bin has no xorriso, openssl or jq.
  tools = [
    # The pinned nightly with rust-src, the UEFI and wasm32 targets, and the
    # std platform layer already applied to a copy of it, so make never has
    # to patch a sysroot or add a target.
    (capsules.mkRustStd pins.rustShell)
    userland.zig
    pins.cmake
    pins.pythonTools
    pins.llvm.clang-unwrapped
    pins.llvm.llvm
    # boot, measure and inspect
    pkgs.qemu
    pins.swtpm
    pkgs.gnutls
    # image assembly and signing
    pkgs.xorriso
    pkgs.mtools
    pkgs.gptfdisk
    pkgs.dosfstools
    pkgs.osslsigncode
    pkgs.openssl
    # How openssl-sys finds the OpenSSL above when make builds sign-kernel, as
    # the flake's own host-tool build does (image.nix). Without it the crate
    # fell back to the host's /usr/include, a different OpenSSL from the
    # library it links.
    pkgs.pkg-config
    pkgs.b3sum
    # the verify page's smoke test and the browser prelude's tests
    pkgs.nodejs
    # the supply chain lane: advisories, licences, the cargo SBOM
    pkgs.cargo-audit
    pkgs.cargo-deny
    pkgs.cargo-cyclonedx
    # the make surface and the scripts it runs
    pkgs.gnumake
    pkgs.git
    pkgs.jq
    pkgs.perl
    pkgs.coreutils
    pkgs.findutils
    # the Linux userland build scripts the make lane runs in this shell: unzip
    # reads gojq's module zips, go builds gojq, ruby builds mruby's mrbc.
    pkgs.unzip
    pins.go
    pkgs.ruby
  ] ++ linuxOnly;
in
pkgs.mkShell {
  name = "nonos";
  packages = tools;
  passthru.tools = tools;

  # The make lanes run only here (the Makefile enters this shell), and read
  # these instead of searching the host.
  NONOS_IN_FLAKE = "1";
  NONOS_PYTHON = "python3";
  NONOS_ZIG = "${userland.zig}/zig";
  NONOS_CMAKE = "${pins.cmake}/bin/cmake";
  NONOS_STD_PATCHED = "1";
  OVMF = "${firmware}/edk2-x86_64-code.fd";
  OVMF_VARS = "${firmware}/edk2-i386-vars.fd";

  shellHook = ''
    echo "NONOS: nix build to build, nix flake check to check (tools/nix/README.md)." >&2
  '';
}
