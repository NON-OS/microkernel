# platform

A host-side command line for packaging a capsule outside the tree's build:
`nonos-capsule` (also built as `nonos`). It runs on the developer's machine with
`std`, not on NONOS.

| Crate | What it does |
|---|---|
| `nonos_capsule` | the CLI: `new`, `build`, `manifest`, `sign`, `install`, `run`, `inspect`, `remove` |
| `nonos_manifest` | parses a project's `Nonos.toml` into a `Manifest` and builds the signer's arguments |
| `nonos_sign_bridge` | runs the hybrid signer, `capsule-sign` unless `NONOS_SIGNER` names another |
| `nonos_package` | the package directory layout: `payload.elf` and `manifest.nmf` |
| `nonos_install` | runs the signer's `verify-manifest` against a certificate and policy, checks the payload against the manifest, then copies the package into the store |
| `nonos_registry` | the local index of installed capsules |

`build` runs `cargo build --release -Zbuild-std=core,alloc` for the manifest's
target. The store is a directory on the host, `$NONOS_STORE` or
`~/.nonos/store`, with `registry.index` beside it; it is not the package store
on a NONOS disk.

Nothing in `mk/`, `tools/nix` or CI builds or runs these crates. A capsule that
ships is built and signed through its `Capsule.mk` instead; see
[adding a capsule](../../docs/handbook/extending/capsule.md).
