# l2_proofs

Host-runnable proofs for `net.l2`'s request header decode. The crate includes
the capsule's own `src/protocol/` (header decode and `refused`) and its reply
path, `src/server/respond/`, by `#[path]` and runs it over a stand-in
`nonos_libc` (`net_proofs/shim`) that keeps each reply, so what is tested is
the code that ships.

- Whatever bytes a client sends, `parse` either accepts a request or refuses
  it, and a refused frame is answered: under the op and request id it names,
  or under zeros when it is too short to name them. Before this the capsule
  dropped such a frame and its caller waited out its whole call timeout.

Not covered here: the rest of the capsule. The ARP parser and the learning rules are in
`net_proofs`.

Run: `cd userland/l2_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-l2_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the network stack](../../docs/handbook/network/stack.md) and
[proofs](../../docs/handbook/verification/proofs.md).
