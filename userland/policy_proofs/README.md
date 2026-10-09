# policy_proofs

Host-runnable proofs for the policy service's request path. Every
settings read and every first-boot answer goes through it, and any
process holding the endpoint can send it any bytes. The real store, the
kernel push, the get and set handlers, the replies and `serve` are
included through `#[path]` from `../capsule_policy/src/`.

`libc_shim/` is a stand-in `nonos_libc`: it answers the kernel calls the
included files make (the service lookup that decides who may set, the
policy push) and keeps each reply for the tests to read.
`wallpaper_catalog_proofs` uses the same shim.

`serve_tests` sends any frame, from any sender, through `serve` to the
real handlers and store, and checks each is answered exactly once, to its
sender, with a whole header naming the op and field it answers.

Run: `cargo test` in this directory. The policy store is described in
[System apps and services](../../docs/handbook/apps/system-apps.md).
