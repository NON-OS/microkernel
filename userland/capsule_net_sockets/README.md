# capsule_net_sockets

## Role

`capsule_net_sockets` is the socket service, the handle `net.sockets`. it gives
application capsules one socket API over IPC and turns each request into calls
on the transports: `net.tcp` for streams, `net.udp` for datagrams, `net.nym`
for mixnet sockets, and `net.dns` for names. the kernel has no socket syscalls;
handles, ownership and dispatch live here.

```text
application capsule (browser, Linux personality, SDK programs)
    |
    | NSKT requests over IPC (MkIpcRecvFrom -> MkIpcReply)
    v
net.sockets -- per-pid handle table --> net.tcp / net.udp / net.nym / net.dns
                                        (MkIpcCall with timeout)
```

it is in the desktop image, where `net.tcp`, `net.udp` and `net.dns` all
resolve to `net.core`, and in the `microkernel-net-sockets`,
`microkernel-net-nym` and `microkernel-input-e2e-ps2` profiles. the capsule
catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md).

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS := 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
  (`0x10`). `CAPSULE_OPTIONAL_CAPS := 0x100`, Debug, granted only by a
  `capsule-serial-debug` build; the kernel mirror
  (`src/userspace/capsule_net_sockets/spawn.rs`) requests `Capability::IPC |
  Capability::Memory | serial_debug_cap() | Capability::Network`.
- service `service:4460:net.sockets`, reply
  `reply:4461:endpoint.net.sockets.reply`, declared in `Capsule.mk`. the feature
  is `nonos-capsule-net-sockets`, namespace `systems.nonos.net.sockets`.
- it serves callers with `MkIpcRecvFrom` (`mk_ipc_recv_from`) and `MkIpcRecv`
  (`mk_ipc_recv`) and answers with `MkIpcReply` (`mk_ipc_reply`). it reaches the
  transports with `MkIpcCall` under a timeout (`mk_ipc_call_timeout`) after
  `MkServiceLookup` (`mk_service_lookup`), checks owners with `MkPidAlive`
  (`mk_pid_alive`), reads the clock with `MkTimeMonotonic` (`mk_uptime_ms`) and
  `MkTimeMillis` (`mk_time_millis`), and waits with `MkYield` (`mk_yield`).

## Interface contract

the capsule serves the `net.sockets` endpoint. requests carry the 20 byte
header shared by the stack capsules, magic `0x4E534B54` ("NSKT"), version 1.

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

handles are keyed by pid and handle; a request answers only its owner. a frame
that fails header parsing is still answered, under the op and request id it
names, or under zeros when it is too short.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
(`0x10`). `CAPSULE_OPTIONAL_CAPS := 0x100`, Debug, granted only by a
`capsule-serial-debug` build. no CoreExec, Crypto, FileSystem or hardware bits.
the kernel requires Network of every sender, so a capsule without it cannot open
a socket here. the kernel installs the mask from the verified manifest at spawn
and the capsule cannot widen it.

## Privacy and persistence

the table holds each socket's owner, kind, addresses and transport handle in
memory. payloads pass through and are not kept beyond what a mixnet read holds
for its caller. nothing is written to storage.

## Runtime lifecycle

spawned by the verified path. it looks up `net.tcp`, `net.udp` and `net.dns`
once at setup, and `net.nym` again on use until it appears. the server loop
takes a request with `MkIpcRecvFrom`, dispatches it, and answers with
`MkIpcReply`. a stream connect does not block the service: it is recorded as
pending and the loop polls `net.tcp` every `PENDING_POLL_MS` until it is up,
fails or eight seconds pass. it is long lived and holds the one endpoint open.

## Failure model

- unknown ops are answered with `E_BAD_OP`, never dropped
  (`src/server/runner.rs`).
- explicit errors for bad handle, table full and wrong socket state; errnos in
  `src/protocol/errno.rs`.
- on a mixnet socket, `OP_CONNECT_HOST` refuses any non-address name with
  `E_NAME_REFUSED` (15) and never asks `net.dns`: a mixnet frame carries an
  address, so a name could only reach the exit after a lookup in the clear.
- a client that ended without closing its sockets has them freed, releasing
  their transports through the close handler's `release`.
- a malformed frame is answered rather than dropped.
- on the desktop, `OP_LISTEN` and `OP_ACCEPT` fail: `net.core` has no listening
  TCP.

## Current implemented surface

the fourteen ops above, served on inbox 0. the per-pid handle table
(`src/sockets/`), the stream, datagram, mixnet and name clients
(`src/clients/`), the pending-connect poller
(`src/server/handlers/connect/pending/`), the mixnet framing
(`src/server/handlers/mixnet_frame/`) and the reaper
(`src/server/handlers/reap.rs`).

## Wire format

the 20 byte little-endian request/reply header, from `src/protocol/header.rs`,
is the one the stack shares:

```text
offset  size  field
0       4     magic = 0x4E534B54 ("NSKT")
4       2     version = 1
6       2     op
8       2     errno (reply only; zero in a request)
10      2     reserved
12      4     request_id
16      4     payload_len
20      ...   payload
```

each op's body follows the header: `OP_SOCKET` names family and kind,
`OP_CONNECT` and `OP_CONNECT_HOST` carry the target, `OP_SEND` and `OP_RECV` the
bytes. a mixnet socket's writes are framed into bodies of at most `MAX_BODY`
bytes before they reach `net.nym`.

## State ownership

the handle table (`src/sockets/table/`) holds each socket's owner pid, kind,
addresses and transport handle, keyed by pid and handle. it holds `TABLE_CAP`,
256, sockets with at most `PER_PID_MAX`, 128, per client. the kernel owns no
socket state; the transports own the connection and port state behind the
handles.

## Operating rules

- scope socket handles to caller identity; a request answers only its owner.
- route all transport work to `net.udp`, `net.tcp`, `net.dns` or `net.nym`.
- return explicit errors for bad handle, table full and wrong socket state.
- hold one client to `PER_PID_MAX` sockets, half the table, so no client can
  refuse every other socket on the machine.
- free the sockets of a client that ended without closing them, releasing their
  transports: looked for every two seconds while requests arrive, and at once
  when a socket cannot be opened (`src/server/handlers/reap.rs`).
- answer unknown ops with `E_BAD_OP`.
- do not introduce Linux-shaped socket syscalls.

## Release target

0.9.2.

## Release evidence

`userland/sockets_proofs` runs the table's per-client share, the take-out of
ended clients' sockets, a random open, close and end run, and the reply a
refused frame gets.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x0001c`.
- [ ] the server answers unknown ops with `E_BAD_OP` (`src/server/runner.rs`).
- [ ] `sockets_proofs` passes.
- [ ] the kernel mirror `src/userspace/capsule_net_sockets` requests the same
      mask plus optional Debug.

## Explicit non-goals today

- it reads no network route: a stream socket here is a direct connection
  whatever network the person chose; callers that follow the choice decide
  before they come here.
- on the desktop, `OP_LISTEN` and `OP_ACCEPT` fail: `net.core` has no listening
  TCP.
- no IPv6, no TLS, no raw sockets.

## Verification

`Capsule.mk` carries the mask and endpoints. the 20 byte header is fixed by
`src/protocol/header.rs`, the ops by `src/protocol/ops.rs`, the table bounds by
`src/sockets/table/`. `sockets_proofs` checks the handle table, the reaper and
the refused-frame reply end to end.
