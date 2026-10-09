# image_paint_proofs

Host-runnable proofs for the toolkit's image decoders (PNG, GIF, BMP) and
the browser's image and paint path: `data:` URIs, the clipped blit, CSS
gradients, mask fades, object positioning and inline SVG sizing. The
toolkit, app_skeleton, `nonos_inflate`, `nonos_base64` and `nonos_brotli`
are dependencies; the browser's own sources compile unchanged through
`#[path]` under `crate::browser::*`. The engine gates its host-only entry
points behind its `harness` feature, which this crate turns on.

The tests run against the PngSuite conformance images in `fixtures/`,
rasters generated in the tests with reference decodes (`png_build.rs`,
`gif_build.rs`, `blit_ref.rs`), and the fuzz reproducers that once hung or
ran the heap out. `tests/png_peak.rs` checks the PNG decoder's peak memory.

Run: `cargo test --release` in this directory. Media decoding is
described in [Audio and media](../../docs/handbook/audio-and-media.md).
