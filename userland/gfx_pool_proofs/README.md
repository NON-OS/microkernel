# gfx_pool_proofs

Host proofs for `nonos_gfx_pool`, the anonymous memory behind every display
driver's primaries (bga, bootfb, the Intel engine, virtio-gpu), mounted by
`#[path]`. `libc_shim` stands in for the kernel's mapping and surface
registry calls by the rules their handlers state: mappings come back
poisoned, munmap takes whole mappings only, register refuses memory that is
not mapped, a share to the owner's own process records no right, and a
receiver named twice keeps one right.

Proved: a primary is whole pages and zero over poisoned ones; a refused
mapping is ENOMEM with nothing made; a scrub zeroes exactly its region; a
primary is registered zero and shared writable with the compositor alone;
with no compositor, or a refused register, nothing is registered; a share
refused after the register gives the surface back; a release withdraws the
surface, then unmaps exactly its pages; a compositor started again under a
new pid is named a writable receiver of the primary the server kept; a
silent owner's primary reaches the next owner zero; a handle the pool does
not hold is shared with no one; and the shared wire sweep on a server over
the real pool leaves no dirty registration, no partial unmap, and exactly
the held primaries mapped and registered.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
