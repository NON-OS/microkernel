# zstd

`nonos_zstd` decodes Zstandard (RFC 8878): raw, RLE and compressed blocks,
Huffman literals, FSE sequence tables in all four modes (predefined, RLE,
compressed and repeat), repeat offsets, and the XXH64 content checksum.
`no_std` with `alloc`, no dependencies. It is written in this tree, not vendored.

## Public surface

```rust
pub const MAX_OUT: usize = 64 * 1024 * 1024;
pub fn decompress(data: &[u8]) -> Option<Vec<u8>>
pub fn xxh64(data: &[u8], seed: u64) -> u64
```

`decompress` decodes every frame in `data` in turn and returns the
concatenated output, or `None` on any failure. Skippable frames are passed
over. A frame's declared content size and checksum must match. A declared
content size over `MAX_OUT` is refused up front. Each block is limited to
128 KiB, and the running total is compared with `MAX_OUT` after each block.
Match offsets may not reach back past the start of the current frame. Empty
input is `None`.

## What it does not do

It does not compress. Dictionaries are not supported: a frame that names a
dictionary ID other than zero is refused. The frame's window descriptor is not
read, so the decoder does not enforce the window size the frame declares. There
is no streaming API and no partial output on error.

## Users

`capsule_linux` decompresses zstd payloads with it
(`src/linux/install/unpacked.rs`), and `capsule_linux_proofs` depends on it
because it compiles that source. The `xz` crate's tests borrow this crate's
input generators and mutator from `tests/support/`.

## Tests

Host tests in `tests/`:

- `vectors.rs`: 12 vectors in `tests/vectors/` decode to exactly their
  regenerated input (`tests/support/table.rs`), covering empty, small and
  400 KB inputs, noise, runs, no checksum, and several frames with a skippable
  frame between. A flipped byte and every truncation of a small vector are
  refused, and `xxh64` matches two known answers.
- `mutate.rs`: seeded mutation of every vector (bit flips, truncation,
  spliced noise, overwritten lengths) and pure noise return without a panic,
  an overflow or output past the bound. Overflow traps need a debug build, so
  it runs without `--release`.

The crate is not a `*_proofs` crate, so `nix flake check` does not run these
tests. No Kani proofs. See
[Linux personality](../../docs/handbook/linux/personality.md).
