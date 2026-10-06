# tcp_proofs

Host-runnable proofs for `net.tcp`. The crate includes the capsule's own source,
everything but its entry point and inbox loop, by `#[path]`, and drives it
through its real request handlers against a peer played on the host behind a
stand-in `nonos_libc`: segments go out through the capsule's IP client and come
back the same way, so the state machine under test is the one that ships.

The tests (`src/tests/`) cover the handshake, passive open and the first ACK of
an accepted connection; RTT and retransmission; the peer's MSS; the ACK range
(RFC 5961 5.2); resets believed only where RFC 9293 and RFC 5961 allow;
segments to or from impossible addresses; overlapping and wrapped segments;
FIN order after our FIN and in CLOSE-WAIT; the persist timer and window
updates; a read bounded by its reply; half-open connections under a SYN flood;
TIME-WAIT and FIN-WAIT-2 lingering; the reset of an ended owner's connections;
hostile input; and the reply a refused frame gets.

Run: `cd userland/tcp_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-tcp_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`. The segment parser and reassembly buffer are
also in `net_proofs`.

See [the network stack](../../docs/handbook/network/stack.md) and
[proofs](../../docs/handbook/verification/proofs.md).
