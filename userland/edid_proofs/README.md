# edid_proofs

Host proofs for `nonos_edid`. The parser's modules are mounted by
`#[path]`, so `cargo test` runs the source the loader and the display
capsules ship. EDIDs are built byte by byte from E-EDID 1.4, CTA-861 and
DisplayID 2.0 layouts: every field of a detailed timing, each established,
standard and CVT code, range limits with their offsets, CTA video, audio
and HDMI blocks, the EEODB count, type VII timings and tiles.

Hostile input: every truncation of a three block EDID, every declared
count, every CTA DTD offset with odd data block headers, and noise in each
kind of extension; none may panic or read past the buffer.

The loader's own `pick::parse_edid` is mounted too, and the crate's
`native()` must give the same answer on built panels and 20,000 noisy base
blocks, so the loader can move onto the crate with its log lines unchanged.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
