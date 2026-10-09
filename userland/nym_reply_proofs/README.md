# nym_reply_proofs

Host-runnable proofs for the way a mixnet reply comes home in `net.nym`. The
crate includes the capsule's own source by `#[path]`: the WebSocket frame
reader, message fragments, reply reassembly, the receive queue, the batched
read's records, the session table and the request header decode. A stand-in
`nonos_libc` carries the replies.

- `reassembly_tests`: fragments rebuilt in any order, copies and losses
  handled, and the bounds on sets, bytes and age held.
- `delivery_tests`: delivery to the reader without cutting a message, and
  batched records a reader can put back together.
- `frame_tests`: gateway frames of any size, including ones larger than the
  buffer, read in their places.
- `surb_tests`: the reply block budget.
- `session_share_tests`: one client holds at most half the session table, and
  an ended client's sessions are closed.
- `refusal_tests`: a frame that fails header parsing is answered.

Run: `cd userland/nym_reply_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-nym_reply_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the Nym mixnet capsule](../../docs/handbook/network/nym.md) and
[proofs](../../docs/handbook/verification/proofs.md).
