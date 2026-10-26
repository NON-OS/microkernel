# No release capsule carries the local test chain's names (P5). Every capsule
# a release image takes (capsules.nix, bySlug) is searched for the RPC name,
# which only a development build compiles in, and for the throwaway TLS root
# a development nonos_tls trusts (tools/devnet/tls): its subject's name and
# its key's SHA-256, the pin src/roots/dev.rs compiles in, read through od so
# a newline byte in it cannot split the search. Every capsule with a
# development twin (dev_cargo_features) is also searched for the address that
# name is dialled at. Another capsule may hold that address for its own
# reasons: tokio-smoke dials QEMU's host by default. The carriers' development
# twins must hold all of them, so a search that stopped finding them fails,
# not passes. scripts/check_devnet_feature.py lets this file name them, and
# only them.
{ pkgs, lib, capsules }:
let
  names = [
    "rpc.devnet.nonos.test"
    "NONOS-devnet-throwaway-root"
  ];
  twinNames = [
    "10.0.2.2"
  ];
  # sha256 of tools/devnet/tls/root_spki.der; a stale value fails the carriers.
  pin = "fca7184fe002e4d959866cfbb8e273f3960031924457fcfdb10544b7f7a4a12d";
  pinBytes = lib.concatMapStrings (i: " ${builtins.substring (i * 2) 2 pin}") (lib.range 0 31);
  # The development capsules that dial the local chain's RPC.
  carriers = [ "nox-settle" ];
  # A twin: a capsule bySlugDev builds again, so its two drvs differ.
  twins = builtins.filter (e: capsules.bySlugDev.${e.slug}.drvPath != capsules.bySlug.${e.slug}.drvPath) capsules.catalogue;
  elf = set: e: "${set.${e.slug}}/${e.bin}";
  holds = name: file: "grep -qaF ${lib.escapeShellArg name} ${file}";
  holdsPin = file: "od -An -v -tx1 ${file} | tr -d '\\n' | grep -qF ${lib.escapeShellArg pinBytes}";
  absent =
    test: what: entries:
    lib.concatMapStrings (e: ''
      if ${test (elf capsules.bySlug e)}; then
        echo "release capsule ${e.slug} carries ${what}" >&2; exit 1
      fi
    '') entries;
  present =
    slug:
    lib.concatMapStrings (f: ''
      ${f.test} || { echo "development ${slug} lacks ${f.what}: the search finds nothing" >&2; exit 1; }
    '') (map (name: { test = holds name "${capsules.bySlugDev.${slug}}/*"; what = name; }) (names ++ twinNames)
      ++ [ { test = holdsPin "${capsules.bySlugDev.${slug}}/*"; what = "the root's pin"; } ]);
in
pkgs.runCommand "release-names" { } ''
  ${lib.concatMapStrings (n: absent (holds n) n capsules.catalogue) names}
  ${absent holdsPin "the throwaway root's pin" capsules.catalogue}
  ${lib.concatMapStrings (n: absent (holds n) n twins) twinNames}
  ${lib.concatMapStrings present carriers}
  touch $out
''
