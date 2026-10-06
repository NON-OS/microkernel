# xz

`nonos_xz` decodes `.xz` files: the container (streams, blocks, index, footer,
stream padding) around LZMA2. `no_std` with `alloc`. It depends on
`nonos_hash` for the SHA-256 check. The LZMA decoder follows Igor Pavlov's
`LzmaSpec.cpp`, as `src/lzma_decode.rs` says. No upstream code is vendored.

## Public surface

```rust
pub const MAX_OUT: usize = 256 * 1024 * 1024;
pub fn decompress(data: &[u8]) -> Option<Vec<u8>>
```

`decompress` returns the concatenated output of every stream in `data`, or
`None` on any failure. It checks the stream header and footer CRCs, that the
footer agrees with the header and the index, each block header's CRC and
declared sizes, the block's check (none, CRC-32, CRC-64 or SHA-256), and that
padding is zero. Reserved check types are refused. Output past `MAX_OUT` is
refused; the total is compared after each LZMA2 chunk, so the buffer can pass
the limit by up to one chunk before the call fails.

## What it does not do

It does not compress. LZMA2 is the only filter: a block with BCJ, delta or any
other filter in its chain is refused, not skipped. Raw `.lzma` files and bare
LZMA2 streams without the container are not accepted. There is no streaming
API and no partial output on error.

## Users

`capsule_linux` decompresses xz payloads with it (`src/linux/install/unpacked.rs`),
and `capsule_linux_proofs` depends on it because it compiles that source.

## Tests

Host tests in `tests/`, sharing the input generators and mutator in
`userland/zstd/tests/support/` by `#[path]`:

- `vectors.rs`: 15 vectors in `tests/vectors/`, written by the reference `xz`,
  decode to exactly their generated input. They cover empty and large inputs,
  each check type, several blocks, non-default `lc`/`lp`/`pb` and a 4 KiB
  dictionary. Concatenated streams with padding decode, a BCJ vector is
  refused, a flipped byte is caught, and every truncation of a small vector is
  refused.
- `mutate.rs`: 12,000 seeded mutants across six vectors return without a
  panic or overflow in a debug build, and any accepted output stays within
  `MAX_OUT`.

The crate is not a `*_proofs` crate, so `nix flake check` does not run these
tests. No Kani proofs. See
[Linux personality](../../docs/handbook/linux/personality.md).
