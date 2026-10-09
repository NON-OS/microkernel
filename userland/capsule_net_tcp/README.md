# capsule_net_tcp

## Role

`capsule_net_tcp` is the TCP layer of the split network stack, the handle
`net.tcp`. it owns the connection table, the TCP state machine, retransmission,
congestion control and the stream buffers. it sits above `net.ip` and below
`net.sockets`, and the kernel holds no TCP state.

```text
net.sockets
    |
    | NTCP requests over IPC (MkIpcRecvFrom -> MkIpcReply)
    v
net.tcp -- TCBs + TCP state machine + timers --> net.ip (MkIpcCall)
    |
    v
net.ip -> net.l2 -> NIC driver
```

the desktop image does not carry it: there `net.core` registers the `net.tcp`
name and answers with smoltcp. it is built into the `microkernel-input-e2e-ps2`
test image. the capsule catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md).

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS := 0x0003c`: Network (`0x04`), IPC (`0x08`), Memory
  (`0x10`) and Crypto (`0x20`). the kernel mirror
  (`src/userspace/capsule_net_tcp/spawn.rs`) requests `Capability::IPC |
  Capability::Memory | Capability::Crypto | Capability::Network`, the same mask.
- service `service:4430:net.tcp`, reply `reply:4431:endpoint.net.tcp.reply`,
  declared in `Capsule.mk`. the feature is `nonos-capsule-net-tcp`, namespace
  `systems.nonos.net.tcp`.
- it serves callers with `MkIpcRecvFrom` (`mk_ipc_recv_from`) on inbox 0 and
  answers with `MkIpcReply` (`mk_ipc_reply`). it sends and receives segments
  through `net.ip` with `MkIpcCall` (`mk_ipc_call`) after one `MkServiceLookup`
  (`mk_service_lookup`), reads the clock with `MkTimeMillis` (`mk_time_millis`),
  finds ended owners with `MkPidAlive` (`mk_pid_alive`), and waits with
  `MkYield` (`mk_yield`).

## Interface contract

the capsule serves the `net.tcp` endpoint. requests carry the 20 byte header
shared by the stack capsules, magic `0x4E544350` ("NTCP"), version 1.

| Op | Value | Meaning |
|---|---|---|
| `OP_HEALTHCHECK` | 1 | liveness |
| `OP_LISTEN` | 2 | passive open |
| `OP_CONNECT` | 3 | active open; blocks until established or 8 s pass |
| `OP_ACCEPT` | 4 | take an established connection off a listener |
| `OP_SEND` | 5 | queue bytes |
| `OP_RECV` | 6 | take received bytes, never more than the reply carries |
| `OP_CLOSE` | 7 | close |
| `OP_SHUTDOWN` | 8 | handled as a close |
| `OP_STATE` | 9 | the connection's state |

a connection answers only its owner. a frame that fails header parsing is still
answered, under the op and request id it names, or under zeros when it is too
short. errnos are in `src/protocol/errno.rs`.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0003c`: Network (`0x04`), IPC (`0x08`), Memory
(`0x10`) and Crypto (`0x20`). Network because the kernel registers a network
service only for a holder of it; Crypto for the random ISS key only. no
CoreExec, FileSystem, Debug or hardware bits. the kernel installs the mask from
the verified manifest at spawn and the capsule cannot widen it.

## Privacy and persistence

connections and their buffers live in memory and end with the process. nothing
is logged or written to storage.

## Runtime lifecycle

spawned by the verified path. `setup` resolves `net.ip` through
`MkServiceLookup`, waits until it is registered, seeds its ISS key from
`crypto_random`, then serves inbox 0. the server loop takes a request with
`MkIpcRecvFrom`, runs it, drives timers on each tick, and answers with
`MkIpcReply`. it is long lived and holds the one endpoint open.

## Failure model

- unknown ops are answered with `E_BAD_OP`, never dropped
  (`src/server/runner.rs`); a dropped frame would leave the caller blocked.
- segments are checked as RFC 9293 and RFC 5961 ask: none to or from an address
  no unicast host can have, no ACK for bytes never sent, a reset believed only
  at exactly `RCV.NXT` and answered elsewhere in the window with a challenge
  ACK.
- the transmit seed is random per key so a forged SYN cannot predict the ISS.
- a malformed frame is answered rather than dropped.
- `OP_CONNECT` blocks the whole server for up to eight seconds while the
  handshake runs; this is a known limit, listed below.

## Current implemented surface

the nine ops above, served on inbox 0. the full connection table and TCP state
machine (`src/tcp/`, `src/state/`), retransmission, Reno-shaped congestion
control, the reassembly buffer, the persist and linger timers, and the `net.ip`
segment client (`src/ip_client/`).

## Wire format

the 20 byte little-endian request/reply header, from `src/protocol/header.rs`,
is the one the stack shares:

```text
offset  size  field
0       4     magic = 0x4E544350 ("NTCP")
4       2     version = 1
6       2     op
8       2     errno (reply only; zero in a request)
10      2     reserved
12      4     request_id
16      4     payload_len
20      ...   payload
```

on the wire to `net.ip` the segments are RFC 9293 TCP: the ISS is `iss_for`,
SipHash-2-4 over the four tuple under a key from `crypto_random`, added to the
clock. `peer_mss` reads the peer's MSS option and `send_mss` never sends a
larger segment, 536 bytes when the SYN named none. window scaling, SACK and
timestamps are not negotiated.

## State ownership

the connection table (`src/state/table/`) holds every TCB, keyed by four tuple,
owned by the pid that opened it. the kernel owns none of it. a listener holds at
most `HALF_OPEN_MAX` (8) connections a SYN opened and the peer has not finished,
the oldest giving way to a new SYN, each ending `HALF_OPEN_MS` (30 s) after its
SYN. TIME-WAIT entries count against no owner. an owner holds at most
`MAX_CONN_PER_PID`, 32, connections. the reassembly buffer holds
`REASM_MAX_SEGS`, 32, segments and drains in sequence order across the 2^32
wrap.

## Operating rules

- scope every connection to the owner pid; a connection answers only its owner.
- hold a listener to `HALF_OPEN_MAX` half-open connections so a forged SYN
  cannot hold a place past its listener's few (`src/state/table/half_open.rs`).
- end a FIN-WAIT-2 entry `FIN_WAIT_2_MS` (60 s) after our FIN was acknowledged
  if the peer never sends its own (`src/state/table/linger.rs`).
- reset and free the connections of an owner that ended without closing them,
  looked for every two seconds while the stack is busy
  (`src/state/table/orphans.rs`, `src/server/orphans.rs`); TIME-WAIT ends by
  itself.
- follow RFC 6298 for retransmission, RTO between 200 ms and 60 s, `MAX_RETX`,
  8, tries; Reno from `INIT_CWND` of three segments.
- answer unknown ops with `E_BAD_OP`.

## Release target

0.9.2.

## Release evidence

`userland/tcp_proofs` drives the real request handlers against a peer played on
the host. `userland/net_proofs` runs the segment parser and the reassembly
buffer.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x0003c`.
- [ ] the server answers unknown ops with `E_BAD_OP` (`src/server/runner.rs`).
- [ ] `tcp_proofs` and `net_proofs` pass.
- [ ] the kernel mirror `src/userspace/capsule_net_tcp` requests the same mask.

## Explicit non-goals today

- `OP_CONNECT` blocks the whole server, for up to eight seconds, while the
  handshake runs.
- no window scaling, SACK or timestamps.
- no TLS, DNS or socket handles: those belong to the callers and `net.sockets`.
- no fault injection in this capsule; the `tcp-chaos` feature of `net.ip` is the
  only one, and nothing here drops segments on purpose.

## Verification

`Capsule.mk` carries the mask and endpoints. the 20 byte header is fixed by
`src/protocol/header.rs`, the ops by `src/protocol/ops.rs`. the ISS, MSS and
timer rules are in `src/tcp/` and `src/state/`, the table bounds in
`src/state/table/`. `tcp_proofs` and `net_proofs` check the behavior end to end.
