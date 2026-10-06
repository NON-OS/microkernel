# nym_topology_proofs

Host-runnable proofs for `net.nym`'s route selection and directory parsing. The
crate includes the capsule's own `topology/draw.rs`, `topology/refresh.rs`,
`json/` readers and parts of `directory_sync/` by `#[path]`.

- `draw_tests`: against candidate counts taken from the live active sets, the
  draw stays in range and does not lean on the front of any list.
- `refresh_tests`: the node list is fetched until it holds a gateway and an
  exit, and again within ten minutes of a fetched list expiring; an image or
  signed list is not replaced on a clock.
- `requester_tests` and `base58_tests`: exit addresses read from the directory.

Run: `cd userland/nym_topology_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-nym_topology_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the Nym mixnet capsule](../../docs/handbook/network/nym.md) and
[proofs](../../docs/handbook/verification/proofs.md).
