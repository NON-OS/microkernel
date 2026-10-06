# sockets_proofs

Host-runnable proofs for net.sockets' socket table, on the real source
(`capsule_net_sockets/src/sockets`, included through `#[path]`).

- One client holds at most `PER_PID_MAX` sockets, half the table, so no one
  client can refuse every other socket on the machine; a closed socket gives
  its share back.
- `take_dead` takes out exactly the sockets of clients that ended without
  closing them, and their slots open again. A living client keeps every one.
- Random clients opening, closing and ending never break a bound, and a key
  is never handed out twice.

- The request header decode (`server/parse_req.rs`) and the reply a refused
  frame gets: whatever bytes a client sends, it is answered, under the op and
  request id it names or under zeros when it is too short.

What is not proven here: that the capsule's reaper releases each taken
socket's transport (`server/handlers/reap.rs` calls the close handler's own
`release`), and that `mk_pid_alive` answers truly. Those need a boot.

Run: `cargo test` in this directory. `nix flake check` runs it as
`proofs-sockets_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the network stack](../../docs/handbook/network/stack.md) and
[proofs](../../docs/handbook/verification/proofs.md).
