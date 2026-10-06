# capsule_net_udp

## Role

`capsule_net_udp` is the UDP layer of the split network stack. It sits above
`net.ip`, gives each client the ports it binds, builds and checks UDP headers,
and queues inbound datagrams per port. `net.dns` and `net.ntp.client` use it,
and `net.sockets` uses it for datagram sockets.

```text
net.dns / net.ntp.client / net.sockets
    |
    | NUDP requests over IPC
    v
net.udp -- UDP build/parse + port table --> net.ip
```

The desktop image does not carry it: there `net.core` registers the `net.udp`
name itself. It is built into the `microkernel-net-udp` through
`microkernel-net-ntp` profiles and the `microkernel-input-e2e-ps2` test image.

## Service and endpoints

- Handle `net.udp`, service endpoint `service:4420:net.udp`, reply endpoint
  `reply:4421:endpoint.net.udp.reply`.
- Kernel mirror `src/userspace/capsule_net_udp`.
- It waits until `net.ip` is registered, then serves inbox 0.

## Interface

Requests carry the 20 byte header shared by the stack capsules, with magic
`0x4E554450` ("NUDP") and version 1.

| Op | Value | Meaning |
|---|---|---|
| `OP_HEALTHCHECK` | 1 | liveness |
| `OP_BIND` | 2 | own a port |
| `OP_UNBIND` | 3 | give a port back |
| `OP_SEND` | 4 | send one datagram from a port the caller owns |
| `OP_RECV` | 5 | take one datagram for a port the caller owns |

- A port has one owner. `insert` refuses a port already bound and holds
  `MAX_BINDS`, 64, with at most `BINDS_PER_PID`, 32, per client
  (`src/state/table.rs`).
- When a bind is refused, `take_dead` frees the ports of every client that
  has ended and the bind is tried once more, so a service that restarts can
  bind its own port again.
- Each port queues up to `RX_RING_DEPTH`, 32, datagrams. `OP_RECV` serves from
  the queue and otherwise pulls one segment from `net.ip`, filing it under its
  destination port.
- A payload is at most `UDP_PAYLOAD_MAX`, 1472 bytes.
- A frame that fails header parsing is still answered, under the op and
  request id it names, or under zeros when it is too short.
- Errnos (`src/protocol/errno.rs`): `E_NO_PORT`, `E_PORT_IN_USE`,
  `E_NO_IP_LINK`, `E_RX_EMPTY` and the header errors.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
(`0x10`). Network because the kernel registers a network service only for a
holder of it. No CoreExec, Crypto, FileSystem, Debug or hardware bits.

## Privacy and persistence

Datagrams wait in memory until their owner reads them. Nothing is logged or
written to storage.

## What it does not do

- No retries, ordering or delivery guarantees: that is UDP.
- No broadcast or multicast group membership.
- It reads `net.ip` only when a caller asks to receive.

## Build and verify

- `make nonos-mk-net-udp`, then `nonos-mk-net-udp-sign` and
  `nonos-mk-net-udp-verify`.
- `userland/udp_proofs` runs the request header decode and the reply a refused
  frame gets. `userland/net_proofs` runs the UDP parser and the bind table,
  its per-client share and the freeing of ended clients' ports.

See [the network stack](../../docs/handbook/network/stack.md).
