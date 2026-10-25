# The development mirror

A development image can install the catalogue's Linux apps from a mirror on
the machine that boots it, where the [Anyone network](../overview/glossary-amnesic-boot.md#anyone-network) cannot be reached. No release image can.

## Release builds never dial a mirror directly

Every download a release image makes goes through `nonos_route_link`, over
the network the person chose; installs go over Anyone. The one other way out
capsule_linux has, a mirror on the local network dialled directly by
address, compiles only with its `dev-mirror` cargo feature:

- the direct dial itself (`net/raw.rs`, `net/raw_io.rs`, `install/http/direct.rs`)
  and the test of a private address (`net/route.rs`, `is_local`);
- the branch of a GET that takes it (`install/http/get.rs`) and the skipped
  wait for Anyone (`install/http/reach.rs`).

A build stops when a setting would name a mirror it should not
(`userland/capsule_linux/build/mirror_fence.rs`, `refused`):

- `NONOS_ALPINE_MIRROR` set to anything, in any build. Alpine's mirror is
  compiled in (`install/mirror.rs`, `AT`): Alpine's CDN by name, or, with
  `dev-mirror`, `10.0.2.2:8080`.
- Without the feature, `NONOS_DEB_MIRROR`, `NONOS_PACMAN_MIRROR` or
  `NONOS_PACKAGE_MIRROR` set to a private or link-local address. A named
  mirror still goes over the route layer.

`scripts/check_dev_mirror.py`, run with the static checks, fails when a piece
of the direct path or the development address loses its
`#[cfg(feature = "dev-mirror")]`, or when the feature is a default or is in
`tools/nix/capsules.json`. Its `--self-test`
shows each rule catches a decoy. The proofs are
`userland/capsule_linux_proofs/src/tests/install/mirror_fence_tests.rs`, with
the mutants in `verification/host_mutants/dev-mirror.json`.

## Which images have it

A development image only. capsule_linux's `Capsule.mk` lists `dev-mirror` in
`CAPSULE_DEV_CARGO_FEATURES`. That puts it in `tools/nix/capsules-dev.json`,
never in `capsules.json`, so only the development capsule set builds
capsule_linux with it (`.#capsules-dev`, which every `<profile>-dev` image
takes). Every release image takes the release set (`tools/nix/image.nix`,
`capsuleSet`), and a release seal handed the development set refuses it
(`tools/nonos_seal/release_set.py`). Neither the development mirror nor the
development capsule set is in any release.

## Installing from it

In a development image, capsule_linux fetches Alpine's packages from
`10.0.2.2:8080`. That is the host, as QEMU's user network shows it to the
guest, and the guest asks with `dl-cdn.alpinelinux.org` as the Host line.
The fetch is dialled directly and logged as
`[LINUX] mirror 10.0.2.2 is on the local network: reached directly`.

The host serves it with `tools/nonos-mirror-relay`
([host tools](host-tools.md#serve-package-mirrors-to-a-guest)), started before the boot:

    python3 tools/nonos-mirror-relay --cache target/mirror-cache

The relay fetches over HTTPS through the host's proxy. A package the market
catalogue lists is served only as the bytes its lock pins
(`linux-catalogue.lock.json`, `B`), or not at all. The guest still checks
Alpine's signature and the market's pin itself.

`--tamper APP` serves that app's package with one byte changed, so a boot
shows the guest refusing bytes its pin does not name.
