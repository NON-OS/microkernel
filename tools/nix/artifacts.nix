# The reproducible artifacts of one configured build, in one tree, with the
# manifest that names every file by hash. This tree is the boundary: two
# builds of one commit and one nonos.toml, on any machine, give the same bytes
# here. What the seal adds (enrollment, signatures) lies outside it.
#
#   nonos-build.json      what was built: the resolved configuration, the
#                         kernel features, the toolchain, and the sha256 of
#                         every file below
#   kernel/nonos-kernel   the kernel ELF, before signing, once the tree holds
#                         the trust set of every capsule it embeds
#   bootloader/           BOOTX64 before enrollment and Secure Boot signing,
#                         once the tree holds the kernel's public keys
#   capsules/<slug>/      every capsule ELF the trust set enrolls
#   linux/                the Linux userland and its data files
#   nonos.cdx.json        the software bill of materials
{ pkgs, lib, pins, capsules, userland, image, sbom }:
cfg:
let
  kernel = image.kernel cfg;
  loader = image.bootloader cfg;
  loaderNote = "not built: the loader compiles in the kernel's public keys and root, and the tree "
    + "has none until ek's first seal is committed (tools/nix/README.md, Seal)";
  kernelReady = image.kernelReady cfg;
  kernelNote = "not built: the kernel embeds the certificate, manifest and trailer of every capsule it "
    + "ships, and the tree has none for ${lib.concatStringsSep ", " (image.unsealed cfg.enabled)} until "
    + "ek's next seal is committed (tools/nix/README.md, Seal)";
  config = builtins.toJSON {
    inherit (cfg) profile about smp install loader rollback_index store features name installer network release taken_out;
    enabled = builtins.filter (f: lib.hasPrefix "nonos-capsule-" f || lib.hasPrefix "microkernel-" f || f == "nonos-smp") cfg.enabled;
    toolchain = {
      rust = pins.rust.version;
      zig = "0.16.0";
      cmake = pins.cmake.version;
      clang = pins.llvm.clang-unwrapped.version;
    };
    # Each toolchain by its store path, whose hash covers every input it was
    # built from. Named, not depended on: the artifacts do not carry them.
    toolchain_paths = lib.mapAttrs (_: p: builtins.unsafeDiscardStringContext (toString p)) {
      rust = pins.rust;
      rust_uefi = pins.rustUefi;
      zig = userland.zig;
      cmake = pins.cmake;
      clang = pins.llvm.clang-unwrapped;
    };
    kernel = if kernelReady then "built" else kernelNote;
    bootloader = if image.loaderReady then "built" else loaderNote;
    epoch = image.epoch;
    commit = image.rev;
  };
in
pkgs.runCommand "nonos-${cfg.name}" {
  inherit config;
  passAsFile = [ "config" ];
  nativeBuildInputs = [ pins.python ];
  passthru = { inherit kernel loader cfg; };
} ''
  mkdir -p $out/kernel $out/bootloader $out/capsules $out/linux
  ${if kernelReady then "cp ${kernel}/nonos-kernel $out/kernel/" else "echo ${lib.escapeShellArg kernelNote} > $out/kernel/README"}
  ${if image.loaderReady then "cp ${loader}/nonos_boot.efi $out/bootloader/" else "echo ${lib.escapeShellArg loaderNote} > $out/bootloader/README"}
  ${lib.concatMapStrings (e: ''
    mkdir -p $out/capsules/${e.slug}
    cp ${capsules.bySlug.${e.slug}}/${e.bin} $out/capsules/${e.slug}/${e.bin}
  '') (builtins.filter (e: !e.dev_only || !cfg.release) capsules.catalogue)}
  # -r: a program can carry its own data directory (Tcl's script library).
  cp -rL ${userland.all}/* $out/linux/
  cp ${sbom} $out/nonos.cdx.json
  cp ${./capsules.json} $out/capsules/catalogue.json
  python3 ${./manifest.py} $out $configPath > $out/nonos-build.json
''
