# wallpaper_catalog_proofs

Host-runnable proofs for the wallpaper catalog service's request path.
The desktop and Settings read wallpapers through it, and any process
holding the endpoint can send it any bytes. The real protocol, catalog,
handlers, replies and `serve` are included through `#[path]` from
`../capsule_wallpaper_catalog/src/`, with `../policy_proofs/libc_shim` as
the stand-in `nonos_libc` that keeps each reply.

`serve_tests` sends any frame, from any sender, through `serve` and
checks each is answered exactly once, to its sender, with a whole header
naming the op and index it answers.

Run: `cargo test` in this directory. The wallpaper capsules are described
in [System apps and services](../../docs/handbook/apps/system-apps.md).
