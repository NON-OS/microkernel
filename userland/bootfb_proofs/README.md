# bootfb_proofs

Host proofs for the boot framebuffer capsule's display half, mounted by
`#[path]` and served through the shared `nonos_gfx_serve` core against a
framebuffer in host memory: the mapping checks (too small, a stride short
of a line, an empty mode, a misaligned base, overflowing sizes), CAPS with
kind 6 and no flags, the one mode, the state; flushes landing in a
blue-first framebuffer as they are and in a red-first one with red and
blue swapped, each fenced; and no mode setting and no cursor (ENOSYS),
with rectangles outside the mode refused.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
