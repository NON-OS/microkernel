# service_header_proofs

Host tests for the request header decode of eight services: `capsule_attest`,
`capsule_desktop_shell`, `capsule_driver_ps2_input`, `capsule_driver_virtio_rng`,
`capsule_entropy`, `capsule_login`, `capsule_power` and `capsule_wallpaper`. Any
process holding one of their endpoints can send any bytes, and the header decode
is the first code those bytes meet. Each `protocol` module is mounted by
`#[path]` under its service's name. A macro gives each service two tests: 200,000 random frames, each either served or answered with the service's own refusal, and a set of boundary frames.

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-service_header_proofs`, with overflow checks on and clippy. See [the proofs page](../../docs/handbook/verification/proofs.md).
