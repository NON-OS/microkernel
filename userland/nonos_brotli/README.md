# nonos_brotli

`nonos_brotli` is a Brotli (RFC 7932) decompressor. `no_std` with `alloc`, no
dependencies. It is written in this tree. The RFC 7932 appendix A static
dictionary ships as `src/dictionary.dat` (122,784 bytes) and is included with
`include_bytes!`.

## Public surface

```rust
pub fn decompress(input: &[u8], max_out: usize) -> Result<Vec<u8>, Error>
```

The whole stream in, the whole output out. Each meta-block's length is checked
against `max_out` before its bytes are reserved, so an output that would pass
the cap is refused with `Error::TooLarge`. Allocation goes through
`try_reserve`, and a failure is `Error::NoMemory`. A cut stream is
`Error::Truncated` and a stream that breaks the format is `Error::Invalid`.
Input after the end of the stream is left unread.

## What it does not do

It does not compress, and it does not stream: there is no incremental API and
no way to get a partial output back. The browser does not use it for
`Content-Encoding: br` yet.

## Users

`capsule_browser` decodes WOFF2 font data with it
(`src/browser/fonts/woff2/decode.rs`). `capsule_browser_proofs` and
`image_paint_proofs` depend on it because they compile the browser's sources.

## Tests

Host tests in `tests/`, run with `cargo test`:

- `vectors.rs`: streams made by the reference encoder (python `brotli` 1.2.0,
  per `tests/common/streams.rs`) and stored in `tests/data/streams.dat` decode
  to their input byte for byte. The set covers every quality 0 to 11, every
  window 10 to 24 and modes 0 to 2. Also: a cap one byte short gives
  `TooLarge`, trailing bytes are ignored, and hand-made streams.
- `malformed.rs`: every cut of a sampled set of streams is refused, and 30,000
  mutated streams either fail or stay inside the cap, with most refused.
- `noise.rs`: 20,000 inputs of random bytes and text return an error or an
  output within the cap, and more than 19,000 of them are refused.

The crate is not a `*_proofs` crate, so `nix flake check` does not run these
tests directly. It runs the WOFF2 tests in `capsule_browser_proofs`
(`src/color_fonts_images/woff2_tests.rs`, `woff2_bad_tests.rs`), which go
through this decoder. No Kani proofs. See
[Browser](../../docs/handbook/apps/browser.md).
