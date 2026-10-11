# gfx_wire_proofs

Host proofs for `nonos_gfx_wire`, mounted by `#[path]`. The header against
section 12's byte offsets; requests held to their declared payload and to
256 bytes; replies held to the request they answer; each body's round trip
and refusals (an adapter with no output, more than 20 modes, a stride
shorter than a row, 0 or 16 rectangles, a cursor past 64); the rectangle
bound without wrapping; serials; the fit adapter; and 20,000 noisy frames
of every length through every decoder.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
