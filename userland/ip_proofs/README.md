# ip_proofs

Host-runnable proofs for `net.ip`'s ingress and ICMP echo path. The crate
includes the capsule's own source, everything but its entry point and inbox
loop, by `#[path]`, and drives it through the real `OP_POLL_PACKET` handler.
`net.l2` is played on the host behind a stand-in `nonos_libc`: frames come up
as they would from the NIC, and what the capsule sends back down is kept.

- `echo`: an echo request to our address is answered and never reaches a
  caller; one to the broadcast address is not answered.
- `sources`: a datagram from 0/8, loopback, 224 and up, or our own address is
  dropped.
- `malformed` and `noise`: a malformed frame, or random bytes, is dropped and
  the poll goes on rather than failing.
- `route`: the longest prefix wins and prefix 0 is the default.
- `refusal`: a frame that fails header parsing is answered.

The `tcp-chaos` feature is declared so its `cfg` resolves and is off, as in
every shipped build.

Run: `cd userland/ip_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-ip_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the network stack](../../docs/handbook/network/stack.md) and
[proofs](../../docs/handbook/verification/proofs.md).
