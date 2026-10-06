# route_proof_proofs

Host proofs for `nonos_route_proof`: the route report wire, who may post one,
and the verdicts drawn from it. The real source is included by `#[path]`, with
the attest service's own protocol decoder (`capsule_attest/src/protocol`).

- `report_tests`: the 40 byte report encodes and decodes exactly, and a short,
  long or wrong-version one is refused.
- `facts_tests` and `nym_facts_tests`: the reports `net.anon` and `net.nym`
  build from their own facts.
- `board_tests`: only `net.nym` about Nym and `net.anon` about Anyone, each
  holding Network, may post; a refused post changes nothing; ages are on the
  board's clock.
- `frame_tests`: the post and the question decode with the attest service's
  decoder, so the two cannot drift apart.
- `verdict_tests`: anonymous only from a fresh report of a route that is up,
  over a quorum-signed and valid directory with every hop authenticated;
  Direct is exposed; anything else is not established, with the reason.

Run: `cd userland/route_proof_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-route_proof_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the Nym mixnet capsule](../../docs/handbook/network/nym.md),
[the Anyone onion routing capsule](../../docs/handbook/network/anyone.md) and
[proofs](../../docs/handbook/verification/proofs.md).
