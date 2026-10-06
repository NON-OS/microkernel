# capsule_socks5

`capsule_socks5` is a SOCKS5 (RFC 1928) front end that carries client streams over the Nym
mixnet through the `net.nym` capsule. Clients speak SOCKS5 to it over IPC; it turns each
CONNECT into a Nym network requester stream. It is `no_std` and depends only on the userland
libc and `spin` (`Cargo.toml`).

## Role

```text
client capsule -- MkIpc (SOCKS5 bytes) --> net.socks5 -- MkIpc --> net.nym -- mixnet --> exit
```

`src/setup.rs` resolves only `net.nym`; its comment says it deliberately does not use
`net.tcp`, so no direct clearnet path exists in this capsule. `src/main.rs` waits until
`net.nym` is found, answering any early request with the closed marker, then enters
`server::run()`.

## Capabilities

From `Capsule.mk`:

```make
CAPSULE_REQUIRED_CAPS    := 0x0001c
CAPSULE_OPTIONAL_CAPS    := 0x100
```

Decoded against `src/capabilities/types/defs.rs`, 0x0001c is Network, IPC and Memory;
the optional 0x100 is Debug (bit 8), which only a `capsule-serial-debug` build grants.
Debug backs the `mk_debug` trace lines in `src/server/trace.rs`. Crypto was held until 0.9.2 and admitted nothing: `net.nym` does the cryptography.

## Interface

- Service endpoint `service:4908:net.socks5`, reply endpoint
  `reply:4909:endpoint.net.socks5.reply`, handle `net.socks5`.
- Each request starts with a marker byte (`src/server/request.rs`): 0 stream bytes, 1 reset,
  2 numbered stream bytes (u32 little endian sequence, then bytes), 3 reset of a named stream
  (u32 stream), 4 numbered bytes on a named stream (u32 stream, u32 sequence, then bytes). A
  stream is never named 0; frames 0 to 2 are stream 0. An empty stream request polls for data
  that has arrived.
- Each reply starts with 0 (open) or 1 (closed) followed by stream bytes
  (`src/server/reply.rs`). A numbered answer is kept and repeated if the caller asks for the
  same number again (`src/server/kept.rs`).
- A conversation is keyed on the kernel-attested pid and the stream it named
  (`src/server/who.rs`), so no caller reaches another's; one caller holds at most
  `STREAMS_PER_CALLER = 8` and keeps that many answers.
- Handshake state is per conversation, up to `MAX_CLIENTS = 32` (`src/server/clients.rs`);
  tunnels are tracked in a fixed table of `MAX_CONNS = 64` (`src/manager/table.rs`).
- Open failures map to distinct SOCKS reply codes (`src/server/open.rs`). A send that
  `net.nym` refuses with `E_NO_SESSION` (it dropped every session when it lost its gateway)
  drops the session here too, and the open is tried once more on a fresh one.
- To `net.nym` it uses ops 3, 4, 5, 7, 17, 19 and 20 (`src/ipc/ops.rs`). Replies are read with
  `OP_RECV_BATCH`, 20, which returns every delivered message that fits as records; a message
  split across answers is put back together by `Joiner` (`src/nym/batch.rs`). Against a
  `net.nym` that does not know op 20 it falls back to `OP_RECV`, one message a call.
- An answer waits at most 40 ms on the exit when nothing has come, and reads the mixnet at
  most 32 times while it has room (`src/server/relay.rs`, `src/server/gather.rs`).
- The reorder buffer holds at most 1 MiB or 1024 chunks per connection and 6 MiB in all; past a
  bound the stream holding the most is ended and its reader told. A gap that has not filled in
  60 s, with later bytes waiting, ends its stream the same way (`src/server/inbox.rs`).
- Every two seconds, and when a new caller finds the table full, the slot, tunnel and kept
  reply of every caller that has ended are freed (`src/server/feed.rs`, `src/server/kept.rs`).

## State and privacy

All state is in capsule memory (`src/server/state.rs`); nothing is persisted. The exit is
chosen in `src/nym/exit.rs`: a configured exit wins, otherwise one from `net.nym`'s directory
(`OP_GET_EXIT`, `src/nym/discover.rs`), otherwise the four compiled-in addresses in
`src/nym/bootstrap.rs`. An exit with an all-zero key or identity is rejected. An exit that has
delivered nothing within `SILENCE_MS = 12_000` of its first send is rotated
(`src/nym/watch.rs`).

`src/server/trace.rs` writes the length of the requested destination to the debug channel
for every open, never the host or port (`destination`, called from `src/server/open.rs`).
The serial log is readable by whoever holds the machine.

## Build and test

- Build: `make nonos-mk-socks5`; sign and verify targets are `nonos-mk-socks5-sign` and
  `nonos-mk-socks5-verify` (from `nonos-mk/capsule.mk`).
- Host tests: `userland/capsule_socks5_proofs` mounts the wire, conn, manager, tunnel and
  watch modules and the server's clients, gather, inbox, kept, reply and request modules, plus
  `nym/batch.rs` and parts of `nonos_route_link`, by `#[path]`. `nix flake check`
  runs it (`tools/nix/checks.nix`), as CI does in `.github/workflows/verify.yml`.

## Not done yet

- `set_exit` in `src/nym/exit.rs` has no caller, so there is no path to configure an exit by
  hand; selection always falls to the directory or the bootstrap list.
- Only the no-auth method and CONNECT are handled; `src/wire/mod.rs` states BIND and UDP
  ASSOCIATE are out of scope, and other commands get `REP_CMD_UNSUPP`.
- The IPC calls and the session code that talk to `net.nym` have no host tests.
- It serves one request at a time, so an answer that waits on the exit holds every other
  caller for that long.

See [the SOCKS5 bridge and network routes](../../docs/handbook/network/socks5-and-routes.md).
