# capsule_net_tcp

## Role

`capsule_net_tcp` is the TCP layer of the split network stack. It owns the
connection table, the TCP state machine, retransmission, congestion control
and the stream buffers. It sits above `net.ip` and below `net.sockets`, and the
kernel holds no TCP state.

```text
net.sockets
    |
    | NTCP requests over IPC
    v
net.tcp -- TCBs + TCP state machine + timers --> net.ip
```

The desktop image does not carry it: there `net.core` registers the `net.tcp`
name and answers with smoltcp. It is built into the `microkernel-input-e2e-ps2`
test image.

## Service and endpoints

- Handle `net.tcp`, service endpoint `service:4430:net.tcp`, reply endpoint
  `reply:4431:endpoint.net.tcp.reply`.
- Kernel mirror `src/userspace/capsule_net_tcp`.
- It waits until `net.ip` is registered, seeds its ISS key, then serves
  inbox 0.

## Interface

Requests carry the 20 byte header shared by the stack capsules, with magic
`0x4E544350` ("NTCP") and version 1.

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

A connection answers only its owner. A frame that fails header parsing is
still answered, under the op and request id it names, or under zeros when it is
too short. Errnos are in `src/protocol/errno.rs`.

## TCP

- The ISS is `iss_for`: SipHash-2-4 over the four tuple under a key from
  `crypto_random`, added to the clock.
- Retransmission follows RFC 6298, with RTO between 200 ms and 60 s and
  `MAX_RETX`, 8, tries. Congestion control is Reno shaped, from `INIT_CWND`
  of three segments.
- `peer_mss` reads the peer's MSS option and `send_mss` never sends a larger
  segment, 536 bytes when the SYN named none. Window scaling is never
  negotiated.
- Segments are checked as RFC 9293 and RFC 5961 ask: none to or from an
  address no unicast host can have, no ACK for bytes never sent, a reset
  believed only at exactly `RCV.NXT` and answered elsewhere in the window
  with a challenge ACK.
- A closed peer window that holds data back is probed by the persist timer,
  and a reader that makes room announces it.
- The reassembly buffer holds `REASM_MAX_SEGS`, 32, segments and drains in
  sequence order across the 2^32 wrap.

## Table rules

- Hold a listener to `HALF_OPEN_MAX` (8) connections a SYN opened and the peer
  has not finished, the oldest giving way to a new SYN, each ending
  `HALF_OPEN_MS` (30 s) after its SYN (`src/state/table/half_open.rs`). Every
  application's sockets reach this capsule as net.sockets, one owner, so a
  forged SYN must never hold a place past its listener's few.
- Count no TIME-WAIT entry against its owner, and give the oldest one's place
  to a new connection when the table is full. End a FIN-WAIT-2 entry
  `FIN_WAIT_2_MS` (60 s) after our FIN was acknowledged if the peer never sends
  its own (`src/state/table/linger.rs`).
- Reset and free the connections of an owner that ended without closing them,
  looked for every two seconds while the stack is busy
  (`src/state/table/orphans.rs`, `src/server/orphans.rs`). TIME-WAIT ends by
  itself.
- An owner holds at most `MAX_CONN_PER_PID`, 32, connections.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0003c`: Network (`0x04`), IPC (`0x08`), Memory
(`0x10`) and Crypto (`0x20`). Network because the kernel registers a network
service only for a holder of it; Crypto for the random ISS key. No CoreExec,
FileSystem, Debug or hardware bits.

## Privacy and persistence

Connections and their buffers live in memory and end with the process.
Nothing is logged or written to storage.

## What it does not do

- `OP_CONNECT` blocks the whole server, for up to eight seconds, while the
  handshake runs.
- No window scaling, SACK or timestamps.
- No TLS, DNS or socket handles: those belong to the callers and `net.sockets`.
- The `tcp-chaos` feature of `net.ip` is the only fault injection; nothing in
  this capsule drops segments on purpose.

## Build and verify

- `make nonos-mk-net-tcp`, then `nonos-mk-net-tcp-sign` and
  `nonos-mk-net-tcp-verify`.
- `userland/tcp_proofs` drives the real request handlers against a peer played
  on the host. `userland/net_proofs` runs the segment parser and the
  reassembly buffer.

See [the network stack](../../docs/handbook/network/stack.md).
