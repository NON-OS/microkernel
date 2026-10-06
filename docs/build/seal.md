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
