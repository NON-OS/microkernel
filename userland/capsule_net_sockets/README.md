# capsule_net_sockets

## Role

`capsule_net_sockets` is the socket service. It gives application capsules one
socket API over IPC and turns each request into calls on the transports:
`net.tcp` for streams, `net.udp` for datagrams, `net.nym` for mixnet sockets,
and `net.dns` for names. The kernel has no socket syscalls; handles, ownership
and dispatch live here.

```text
application capsule (browser, Linux personality, SDK programs)
    |
    | NSKT requests over IPC
    v
net.sockets -- per-pid handle table --> net.tcp / net.udp / net.nym / net.dns
```

It is in the desktop image, where `net.tcp`, `net.udp` and `net.dns` all
resolve to `net.core`, and in the `microkernel-net-sockets`,
`microkernel-net-nym` and `microkernel-input-e2e-ps2` profiles.

## Service and endpoints

- Handle `net.sockets`, service endpoint `service:4460:net.sockets`, reply
  endpoint `reply:4461:endpoint.net.sockets.reply`.
- Kernel mirror `src/userspace/capsule_net_sockets`.
- It looks up `net.tcp`, `net.udp` and `net.dns` once at setup, and `net.nym`
  again on use until it appears.

## Interface

Requests carry the 20 byte header shared by the stack capsules, with magic
`0x4E534B54` ("NSKT") and version 1.

| Op | Value | Meaning |
|---|---|---|
| `OP_HEALTHCHECK` | 1 | liveness |
| `OP_SOCKET` | 2 | a handle: family 4, kind 1 stream, 2 datagram, 3 mixnet |
| `OP_BIND` | 3 | bind a local port |
| `OP_LISTEN` | 4 | listen on a stream socket |
| `OP_ACCEPT` | 5 | take a connection off a listener |
| `OP_CONNECT` | 6 | connect to an address |
| `OP_SEND` | 7 | send bytes |
| `OP_RECV` | 8 | receive bytes |
| `OP_CLOSE` | 9 | release the handle and its transport |
| `OP_GETSOCKOPT` | 10 | read an option |
| `OP_SETSOCKOPT` | 11 | set an option |
| `OP_CONNECT_HOST` | 12 | connect to a host name |
| `OP_POLL` | 13 | readable and writable state |
| `OP_CONNECT_NB` | 14 | connect without waiting |

- Handles are keyed by pid and handle. The table holds `TABLE_CAP`, 256, with
  at most `PER_PID_MAX`, 128, per client.
- A stream connect does not block the service: it is recorded as pending and
  the loop polls `net.tcp` every `PENDING_POLL_MS` until it is up, fails or
  eight seconds pass.
- `OP_CONNECT_HOST` uses the name as an address when it parses as one. On a
  mixnet socket it refuses any other name with `E_NAME_REFUSED` (15) and never
  asks `net.dns`: a mixnet frame carries an address, so the name could only
  reach the exit after a lookup in the clear. Stream and datagram sockets
  resolve through `net.dns`.
- A mixnet socket opens a session on `net.nym`, and its writes are framed into
  bodies of at most `MAX_BODY` bytes.
- A frame that fails header parsing is still answered, under the op and
  request id it names, or under zeros when it is too short.

## Operating rules

- Scope socket handles to caller identity.
- Route all transport work to `net.udp`, `net.tcp`, `net.dns` or `net.nym`.
- Return explicit errors for bad handle, table full and wrong socket state.
- Hold one client to `PER_PID_MAX` sockets, half the table, so no client can
  refuse every other socket on the machine.
- Free the sockets of a client that ended without closing them, releasing their
  transports through the close handler's own `release`: looked for every two
  seconds while requests arrive, and at once when a socket cannot be opened
  (`src/server/handlers/reap.rs`).
- Do not introduce Linux-shaped socket syscalls.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
(`0x10`). `CAPSULE_OPTIONAL_CAPS := 0x100`, Debug, granted only by a
`capsule-serial-debug` build. No CoreExec, Crypto, FileSystem or hardware bits.
The kernel requires Network of every sender, so a capsule without it cannot
open a socket here.

## Privacy and persistence

The table holds each socket's owner, kind, addresses and transport handle in
memory. Payloads pass through and are not kept beyond what a mixnet read holds
for its caller. Nothing is written to storage.

## What it does not do

- It reads no network route. A stream socket here is a direct connection
  whatever network the person chose; callers that follow the choice decide
  before they come here (see the routes page below).
- On the desktop, `OP_LISTEN` and `OP_ACCEPT` fail: `net.core` has no listening
  TCP.
- No IPv6, no TLS, no raw sockets.

## Build and verify

- `make nonos-mk-net-sockets`, then `nonos-mk-net-sockets-sign` and
  `nonos-mk-net-sockets-verify`.
- `userland/sockets_proofs` runs the table's per-client share, the take-out of
  ended clients' sockets, a random open, close and end run, and the reply a
  refused frame gets.

See [the network stack](../../docs/handbook/network/stack.md) and
[network routes](../../docs/handbook/network/socks5-and-routes.md).
