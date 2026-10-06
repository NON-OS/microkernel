# net_anon_proofs

Host-runnable proofs for `net.anon`'s request header decode. The crate includes
the capsule's own `server/parse_req.rs`, `server/respond.rs`, `protocol/` and
the cell geometry and circuit window it names, by `#[path]`, over the stand-in
`nonos_libc` of `net_proofs/shim`, which keeps each reply.

- Whatever bytes a client sends, a frame that does not parse is answered
  rather than dropped. The capsule answers it under `E_BAD_OP` and request id
  0; the tests pin that.

Not covered here: everything else in `net.anon`, which `anon_ntor_proofs` and
`anon_link_proofs` hold.

Run: `cd userland/net_anon_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-net_anon_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the Anyone onion routing capsule](../../docs/handbook/network/anyone.md) and
[proofs](../../docs/handbook/verification/proofs.md).
