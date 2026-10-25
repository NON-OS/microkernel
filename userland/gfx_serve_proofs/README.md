# gfx_serve_proofs

Host proofs for `nonos_gfx_serve`, mounted by `#[path]`. Requests are built
as the compositor builds them; the server answers against `model/vram`, a
VRAM device with a range or a list of modes and an optional cursor, and
`model/pool`, a display pool and surface registry that refuses a double
release and a share of an unregistered surface.

Proved: the first SURFACE makes and shares the primary at the shown mode;
a restarted compositor gets it and frees the primary waiting for RETIRE;
a full pool is ENOMEM with nothing left behind; MODE_SET makes before it
breaks, one waiting primary at a time, RETIRE's three answers; sizes
outside the range or list refused with nothing made; a failing mode set
keeps the old mode and frees its attempt; FLUSH copies exactly its
rectangles and refuses stale serials and outside rectangles untouched;
CURSOR from the slot only with a hardware cursor; CAPS, MODES, STATE,
HEALTHCHECK; ENXIO, ENOSYS and EINVAL; and the copy across strides with
the red and blue swap, skipping a rectangle past either buffer.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
