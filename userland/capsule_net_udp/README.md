# capsule_net_udp

## Role

`capsule_net_udp` is the UDP layer of the split network stack, the handle
`net.udp`. it sits above `net.ip`, gives each client the ports it binds, builds
and checks UDP headers, and queues inbound datagrams per port. `net.dns` and
`net.ntp.client` use it, and `net.sockets` uses it for datagram sockets. the
kernel holds no UDP state.

```text
net.dns / net.ntp.client / net.sockets
    |
    | NUDP requests over IPC (MkIpcRecvFrom -> MkIpcReply)
    v
net.udp -- UDP build/parse + per-port rx ring --> net.ip (MkIpcCall)
    |
    v
net.ip -> net.l2 -> NIC driver
```

the desktop image does not carry it: there `net.core` registers the `net.udp`
name itself. it is built into the `microkernel-net-udp` through
`microkernel-net-ntp` profiles and the `microkernel-input-e2e-ps2` test image.
the capsule catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md).

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS := 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
  (`0x10`). the kernel mirror (`src/userspace/capsule_net_udp/spawn.rs`)
  requests `Capability::IPC | Capability::Memory | Capability::Network`, the
  same mask.
- service `service:4420:net.udp`, reply `reply:4421:endpoint.net.udp.reply`,
  declared in `Capsule.mk`. the feature is `nonos-capsule-net-udp`, namespace
  `systems.nonos.net.udp`.
- it serves callers with `MkIpcRecvFrom` (`mk_ipc_recv_from`) on inbox 0 and
  answers with `MkIpcReply` (`mk_ipc_reply`). it reaches `net.ip` with
  `MkIpcCall` (`mk_ipc_call`) after one `MkServiceLookup` (`mk_service_lookup`),
  and uses `MkPidAlive` (`mk_pid_alive`) to find clients that ended and
  `MkYield` (`mk_yield`) to wait.

## Interface contract

the capsule serves the `net.udp` endpoint. requests carry the 20 byte header
shared by the stack capsules, magic `0x4E554450` ("NUDP"), version 1.

| Op | Value | Meaning |
|---|---|---|
| `OP_HEALTHCHECK` | 1 | liveness |
| `OP_BIND` | 2 | own a port |
| `OP_UNBIND` | 3 | give a port back |
| `OP_SEND` | 4 | send one datagram from a port the caller owns |
| `OP_RECV` | 5 | take one datagram for a port the caller owns |

a port has one owner; `OP_SEND` and `OP_RECV` from any other caller are
refused. a frame that fails header parsing is still answered, under the op and
request id it names, or under zeros when it is too short.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
(`0x10`). Network because the kernel registers a network service only for a
holder of it, so a caller without it cannot send here. no CoreExec, Crypto,
FileSystem, Debug or hardware bits. the kernel installs the mask from the
verified manifest at spawn and the capsule cannot widen it.

## Privacy and persistence

datagrams wait in memory until their owner reads them. nothing is logged or
written to storage, and no payload outlives the process.

## Runtime lifecycle

spawned by the verified path. `setup` resolves `net.ip` through
`MkServiceLookup` and waits until it is registered, then the server loop takes
requests off inbox 0 with `MkIpcRecvFrom`, serves each one, and answers with
`MkIpcReply`. it is long lived and holds the one endpoint open; it is not
restarted per request.

## Failure model

- unknown ops are answered with `E_BAD_OP`, never dropped; a dropped frame
  would leave the caller blocked forever.
- errnos (`src/protocol/errno.rs`): `E_NO_PORT`, `E_PORT_IN_USE`,
  `E_NO_IP_LINK`, `E_RX_EMPTY`, and the header errors `E_BAD_MAGIC`,
  `E_BAD_VERSION`, `E_BAD_LEN`.
- when a bind is refused, `take_dead` frees the ports of every client that has
  ended and the bind is tried once more, so a service that restarts can bind
  its own port again.
- a malformed frame is answered rather than dropped.

## Current implemented surface

the five ops above, served on inbox 0. UDP header build and parse with
checksum, the per-port rx ring, and the `net.ip` client that sends and pulls
one segment on demand (`src/ip_client/`). nothing else: no second endpoint.

## Wire format

the 20 byte little-endian request/reply header, from `src/protocol/header.rs`:

```text
offset  size  field
0       4     magic = 0x4E554450 ("NUDP")
4       2     version = 1
6       2     op
8       2     errno (reply only; zero in a request)
10      2     reserved
12      4     request_id
16      4     payload_len
20      ...   payload
```

a payload is at most `UDP_PAYLOAD_MAX`, 1472 bytes; the IPC body cap is
`IPC_PAYLOAD_MAX`, 1536. the `net.ip` leg carries the IP four tuple and the UDP
datagram.

## State ownership

the bind table (`src/state/table.rs`) holds each port's owner pid and its rx
ring. a port has one owner; `insert` refuses a port already bound and holds
`MAX_BINDS`, 64, with at most `BINDS_PER_PID`, 32, per client. each port queues
up to `RX_RING_DEPTH`, 32, datagrams. the kernel owns none of this.

## Operating rules

- one owner per port; refuse `OP_SEND` and `OP_RECV` from any other caller.
- hold one client to `BINDS_PER_PID`, 32, half the table, so no client can take
  every port on the machine.
- free the ports of a client that ended without unbinding them, looked for when
  a bind is refused (`take_dead`).
- `OP_RECV` serves from the port's ring and otherwise pulls one segment from
  `net.ip`, filing it under its destination port.
- read `net.ip` only when a caller asks to receive.
- answer unknown ops with `E_BAD_OP`.

## Release target

0.9.2.

## Release evidence

`userland/udp_proofs` runs the request header decode and the reply a refused
frame gets. `userland/net_proofs` runs the UDP parser and the bind table, its
per-client share and the freeing of ended clients' ports.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x0001c`.
- [ ] the server answers unknown ops with `E_BAD_OP` (`src/server/runner.rs`).
- [ ] `udp_proofs` and `net_proofs` pass.
- [ ] the kernel mirror `src/userspace/capsule_net_udp` requests the same mask.

## Explicit non-goals today

- no retries, ordering or delivery guarantees: that is UDP.
- no broadcast or multicast group membership.
- no IPv6.
- it holds no network route: a caller that follows the person's network choice
  decides before it sends here.

## Verification

`Capsule.mk` carries the mask and endpoints. the 20 byte header is fixed by
`src/protocol/header.rs`, the ops by `src/protocol/ops.rs`, the limits by
`src/protocol/limits.rs`, and the bind table bounds by `src/state/table.rs`.
`udp_proofs` and `net_proofs` check the behavior end to end.
