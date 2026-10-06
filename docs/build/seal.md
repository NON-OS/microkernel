# The seal

See what turns the build's unsigned artifacts into a bootable image, what it signs and writes, and how to get an image you can boot without the release keys.

## What the seal is

The [seal](../overview/glossary.md#seal) is the one step between the reproducible build and a bootable image. It adds what only the keys can add, and it compiles nothing itself: every artifact it seals comes from a `nix build` of the tree, and the build never holds a key (`seal`, `Makefile:11-14`).

```
make seal
make seal PROFILE=hardened SEAL_ARGS=--release
nix run .#seal -- --profile hardened --release
```

Not tested in this release.

`make seal` runs `nix run .#seal`, which runs `tools/nonos-seal` with the flake's tools on `PATH` (`seal`, `tools/nix/apps.nix:40`). Run any other way, it stops and says to run it as `nix run .#seal` from the checkout (`preflight`, `tools/nonos_seal/__main__.py:42-44`).

| option | effect |
|---|---|
| `--profile P` | seals profile `P` instead of the one `nonos.toml` names |
| `--release` | a release: refuses a dirty tree and a development loader, and, for a profile whose loader policy is `production`, a missing Secure Boot db key |
| `--out DIR` | where the sealed image goes, `target/release` by default |
| `--mirror URL` | the NONOS model repository the model catalogue names |

The options are defined in `main` (`tools/nonos_seal/__main__.py:100-107`).

## Who runs it

The maintainers run the release seal on their own machine, where the release keys are. The kernel's private signing keys are not committed; the seal copies only their public halves into the tree for the loader to compile in (`kernelKeys`, `tools/nix/image.nix:127-134`). The `release` bundle CI publishes is unsigned, and the workflow that builds it holds no key (`release`, `.github/workflows/ci-release-artifacts.yml:3-5`); the `release` workflow then signs build provenance over the release assets (`attest-build-provenance`, `.github/workflows/release.yml:110-113`). Other CI lanes, in production trust mode, read an Ed25519 seed from the `SIGNING_KEY_BASE64` repository secret and stop without it (`SIGNING_KEY_BASE64`, `nonos-ci/setup-signing-key.sh:22-29`). What that secret holds is a repository setting, not visible in the tree.

With `--release`, the seal refuses a tree that differs from the commit outside `nonos-data/trust/`, `nonos-data/market/` and `nonos-data/models/`, where it writes its own output (`preflight`, `tools/nonos_seal/__main__.py:45-49`), and refuses any profile with the development loader (`release`, `tools/nonos_seal/__main__.py:119-120`).
