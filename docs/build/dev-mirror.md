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

Without the feature, a build that names such a mirror stops
(`userland/capsule_linux/build/mirror_fence.rs`, `refused`):

- `NONOS_ALPINE_MIRROR` set to anything, since only a development image sets it;
- `NONOS_DEB_MIRROR`, `NONOS_PACMAN_MIRROR` or `NONOS_PACKAGE_MIRROR` set to a
  private or link-local address. A named mirror still goes over the route layer.

`scripts/check_dev_mirror.py`, run with the static checks, fails when a piece
of the direct path loses its `#[cfg(feature = "dev-mirror")]`, or when the
feature is a default or is in `tools/nix/capsules.json`. Its `--self-test`
shows each rule catches a decoy. The proofs are
`userland/capsule_linux_proofs/src/tests/install/mirror_fence_tests.rs`, with
the mutants in `verification/host_mutants/dev-mirror.json`.
