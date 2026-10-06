# Reproducible builds

Check a NONOS build yourself: what is pinned, where the reproducible boundary runs, how to compare two builds, and what is not reproducible yet.

## The boundary

The artifacts in `result/` are built to be reproducible: the same commit and the same `nonos.toml` should give the same bytes on any machine. Enrollment and signing are not, because the [seal](../overview/glossary.md#seal) draws fresh randomness for every STARK proof and signs with keys the build never sees (`BOUNDARY`, `tools/nix/manifest.py:25-29`). The boundary is the tree the flake's `artifacts` function writes: the kernel ELF, every [capsule](../overview/glossary.md#capsule) ELF, the Linux userland, the loader EFI binary and the bill of materials (`artifacts`, `tools/nix/artifacts.nix:1-15`).

The sealed image still traces back to the source. The seal's last check rebuilds the kernel and the loader from the tree with `nix build` and stops unless they are the bytes it sealed (`reproduced`, `tools/nonos_seal/verify.py:70-86`).
