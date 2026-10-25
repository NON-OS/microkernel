# No release capsule carries the local test chain's names (P5). Every capsule
# a release image takes (capsules.nix, bySlug) is searched for the RPC name,
# which only a development build compiles in, and every capsule with a
# development twin (dev_cargo_features) also for the address that name is
# dialled at. Another capsule may hold that address for its own reasons:
# tokio-smoke dials QEMU's host by default. The carriers' development twins
# must hold both, so a search that stopped finding them fails, not passes.
# scripts/check_devnet_feature.py lets this file name them, and only them.
{ pkgs, lib, capsules }:
let
  names = [
    "rpc.devnet.nonos.test"
    "10.0.2.2"
  ];
  # The development capsules that dial the local chain's RPC.
  carriers = [ "nox-settle" ];
  # A twin: a capsule bySlugDev builds again, so its two drvs differ.
  twins = builtins.filter (e: capsules.bySlugDev.${e.slug}.drvPath != capsules.bySlug.${e.slug}.drvPath) capsules.catalogue;
  elf = set: e: "${set.${e.slug}}/${e.bin}";
  absent =
    name: entries:
    lib.concatMapStrings (e: ''
      if grep -qaF ${lib.escapeShellArg name} ${elf capsules.bySlug e}; then
        echo "release capsule ${e.slug} carries ${name}" >&2; exit 1
      fi
    '') entries;
  present =
    slug:
    lib.concatMapStrings (name: ''
      grep -qaF ${lib.escapeShellArg name} ${capsules.bySlugDev.${slug}}/* \
        || { echo "development ${slug} lacks ${name}: the search finds nothing" >&2; exit 1; }
    '') names;
in
pkgs.runCommand "release-names" { } ''
  ${absent (builtins.head names) capsules.catalogue}
  ${lib.concatMapStrings (n: absent n twins) (builtins.tail names)}
  ${lib.concatMapStrings present carriers}
  touch $out
''
