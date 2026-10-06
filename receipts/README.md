# Receipts

What the build proved, committed beside the source it was proved from.

- `<profile>.json` is written by `make build`: every artifact by sha256, the
  toolchain by the store path that pins it, every pinned input by hash, the
  installer the image carries, what the seal enrolls and signs, and whether
  the build reproduced the receipt committed before it.
- `check-<system>.json` is written by `make check`: every flake check, passed
  or failed, the tests each proof crate ran, and the bill of materials.

Commit a receipt from a clean tree. Anyone can rebuild the same commit and
compare: the artifacts section must match byte for byte.
