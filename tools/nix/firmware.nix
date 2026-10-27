# The firmware package: files a Wi-Fi card runs that no capsule or kernel
# image carries. Each is linux-firmware's at the pinned commit, fetched by its
# sha256 (firmware.json, held to the driver's own pins by
# scripts/check_mediatek_firmware.py), with its licence beside it. The image
# places this directory at EFI/nonos/firmware/mediatek on the boot medium,
# where the loader reads it (scripts/firmware_package.py does the same for
# `make nonos-mk-esp`).
{ pkgs, lib }:
let
  pin = builtins.fromJSON (builtins.readFile ./firmware.json);
  upstream = "https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/plain";
  fetch = f: pkgs.fetchurl {
    name = baseNameOf f.path;
    url = "${upstream}/${f.path}?id=${pin.commit}";
    sha256 = f.sha256;
  };
in
pkgs.runCommand "nonos-firmware-mediatek" { } ''
  mkdir -p $out
  ${lib.concatMapStrings (f: "cp ${fetch f} $out/${baseNameOf f.path}\n") pin.files}
''
