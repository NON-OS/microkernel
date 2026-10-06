# dns_proofs

Host-runnable proofs for `net.dns`'s exchange with its upstream server. The
crate includes the capsule's own source, everything but its entry point and
inbox loop, by `#[path]`, and drives it through its real resolve handlers.
`net.udp` and the upstream resolver are played on the host behind a stand-in
`nonos_libc` (`src/upstream.rs`).

- `exchange`: only a response from the upstream address and port 53, with this
  query's id and question, is read; a stray, forged or malformed packet is
  ignored and the wait goes on.
- `cache_ttl`: an answer is cached for its TTL and never longer than a day.
- `refusal`: a frame that fails header parsing is answered.

The response parser, the name reader and the CNAME chain are in `net_proofs`.

Run: `cd userland/dns_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-dns_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the network stack](../../docs/handbook/network/stack.md) and
[proofs](../../docs/handbook/verification/proofs.md).
