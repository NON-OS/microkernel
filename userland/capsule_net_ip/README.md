# capsule_net_ip

## Role

`capsule_net_ip` is the IPv4 layer of the split network stack. it owns the
interface address and the route table, builds outbound IPv4 packets and hands
them to `net.l2`, and checks inbound packets before passing them up to
`net.udp` and `net.tcp`. it answers ICMP echo requests itself and holds no
hardware authority: the NIC-side authority lives one layer down at `net.l2`.

```text
net.udp / net.tcp
    |
    | net.ip requests over MkIpc (20-byte "NIP4" envelope)
    v
net.ip -- IPv4 parse/build + route table + ICMP echo --> net.l2
                                                         (MkIpcCall to net.l2)
```

the desktop image does not carry it: there `net.core` registers the `net.ip`
name itself and answers ICMP only. it is built into the `microkernel-net-ip`
through `microkernel-net-ntp` profiles and the `microkernel-input-e2e-ps2`
test image.

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
  (`0x10`). the kernel mirror (`src/userspace/capsule_net_ip/spawn.rs`) requests
  exactly `Capability::IPC | Capability::Memory | Capability::Network`. no
  optional caps.
- service `service:4402:net.ip`, reply `reply:4403:endpoint.net.ip.reply`,
  declared for the manifest; the kernel registers the `net.ip` name at spawn.
- the feature is `nonos-capsule-net-ip`, namespace `systems.nonos.net.ip`.
- it reaches the kernel only over the `MkIpc*` syscall surface. it serves
  requests with `MkIpcRecvFrom` (`mk_ipc_recv_from`) and answers with
  `MkIpcReply` (`mk_ipc_reply`); it resolves `net.l2` with `MkServiceLookup`
  (`mk_service_lookup`) and sends and polls frames through it with `MkIpcCall`
  (`mk_ipc_call`). it paces bring-up with `MkYield` (`mk_yield`) and calls
  `MkExit` (`mk_exit`) only when the heap fails to initialise.

## Interface contract

requests carry the 20-byte header shared by the stack capsules, magic
`0x4E495034` ("NIP4"), version 1 (`src/protocol/header.rs`). the op set
(`src/protocol/ops.rs`):

| Op | Value | Who may call | Meaning |
|---|---|---|---|
| `OP_HEALTHCHECK` | 1 | anyone | liveness |
| `OP_GET_CONFIG` | 2 | anyone | address, prefix and gateway |
| `OP_SET_CONFIG` | 3 | `net.dhcp.client` | install a lease and its default route |
| `OP_SEND_PACKET` | 4 | anyone | build and send one IPv4 packet |
| `OP_POLL_PACKET` | 5 | anyone | the next inbound packet of one protocol |
| `OP_ROUTE_ADD` | 6 | `net.admin` | add a route |
| `OP_ROUTE_CLEAR` | 7 | `net.admin` | empty the table |

`src/server/authz.rs` compares the sender's pid with the pid the registry bound
to the named service. no capsule registers `net.admin`, so `OP_ROUTE_ADD` and
`OP_ROUTE_CLEAR` answer `E_PERM` to everyone.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x0001c` is the whole authority the capsule asks for:
Network, IPC and Memory. Network because the kernel registers a network service
only for a holder of it. no CoreExec, Crypto, FileSystem, Debug or hardware
bits: the card belongs to the NIC driver capsule, reached indirectly through
`net.l2`. the kernel installs the mask from the verified manifest at spawn and
the capsule cannot widen it.

## Privacy and persistence

the capsule holds the interface address, prefix, gateway and route table in
memory. packets pass through and are not kept or logged. it writes nothing to
storage and holds no secret.

## Runtime lifecycle

`_start` (`src/main.rs`) initialises the heap, then loops `setup::run` behind
`MkYield` until `net.l2` is registered (resolved by `MkServiceLookup`,
`src/setup.rs`). it then serves inbox 0 forever (`src/server/runner.rs`), one
request at a time. it is not restarted.

## Failure model

a frame that fails header parsing is still answered, under the op and request id
it names, or under zeros when it is too short. a caller that is not the bound
owner of the named service gets `E_PERM`, which is every caller for the two
`net.admin` ops. egress with no address configured is refused. on ingress,
`parse` rejects a bad version, length or checksum and any fragment, and
`from_frame` drops a packet whose source no host can have or one addressed to
someone else; a malformed frame is dropped and the poll goes on. if the heap
cannot initialise at start, `_start` calls `mk_exit(1)`.

## Current implemented surface

the seven ops above, each with a handler in `src/server/handlers`. egress
(`src/egress`): `send` takes the next hop from the longest matching route
(prefix 0 is the default), resolves it through `net.l2` and sends the frame; the
table holds `TABLE_CAP`, 16 routes. ingress happens inside `OP_POLL_PACKET`: up
to `POLL_BUDGET`, eight, frames are pulled from `net.l2` per call; options are
carried but not decoded. an echo request to our address is answered by
`try_reply` (`src/icmp`) and never reaches a caller; one to the broadcast
address is not answered. the `tcp-chaos` cargo feature, off in every shipped
build, drops inbound TCP segments 3 and 6 to exercise retransmission.

## Wire format

the 20-byte v1 envelope (`src/protocol/header.rs`): `u32` magic
(`0x4E495034`), `u16` version (1), `u16` op, `u16` flags (unused on a request,
errno on a response), `u16` reserved, `u32` request_id, `u32` payload_len, then
the payload. the payload of `OP_SET_CONFIG` carries the lease fields; of
`OP_SEND_PACKET` and `OP_POLL_PACKET`, one IPv4 packet. downstream it wraps the
same-shape `net.l2` "NL2\0" envelope, reached with `MkIpcCall`.

## State ownership

the capsule owns the interface address, prefix and gateway (`src/state/iface.rs`),
the route table of up to `TABLE_CAP` entries (`src/route`), and the inbound
packet queue (`src/state/rx_queue.rs`, `src/state/packet.rs`). all of it lives
in memory for the life of the process. the route table changes only through
`OP_SET_CONFIG`, since nothing holds `net.admin`.

## Operating rules

- it reads `net.l2` only when a caller polls: nothing is received while no one
  asks.
- the default route is installed by `OP_SET_CONFIG` from `net.dhcp.client`; the
  `net.admin` route ops are inert until something registers that name.
- ICMP echo to our own address is handled in-capsule and never surfaces to a
  caller.
- one interface, the one `net.l2` bound.

## Release target

0.9.2.

## Release evidence

`userland/ip_proofs` drives the real ingress and ICMP echo path against frames
played from the host. `userland/net_proofs` runs the IPv4 and ICMP parsers. the
`microkernel-net-ip` through `microkernel-net-ntp` profiles and the
`microkernel-input-e2e-ps2` image boot the capsule above a real `net.l2`.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x0001c`, kernel mirror requests the
      same three caps.
- [ ] the seven ops answer, and authz refuses `OP_SET_CONFIG` to any sender but
      `net.dhcp.client` and both route ops to everyone.
- [ ] ingress rejects bad version, length, checksum, fragments and spoofed or
      misaddressed sources; ICMP echo to our address is answered.
- [ ] `tcp-chaos` is off in the shipped build.
- [ ] `ip_proofs` and `net_proofs` pass.

## Explicit non-goals today

- no IPv6, no fragmentation or reassembly, no forwarding between interfaces.
- no socket layer: `net.udp` and `net.tcp` sit above.
- no dynamic routing: the table changes only through `OP_SET_CONFIG`.
- no hardware access of its own.

## Verification

the magic, version and 20-byte layout are fixed in `src/protocol/header.rs`; the
op set in `src/protocol/ops.rs`; the cap mask in `Capsule.mk` and
`src/userspace/capsule_net_ip/spawn.rs`. the `MkIpc*` calls are the only kernel
surface the capsule touches (`mk_ipc_recv_from`, `mk_ipc_reply`, `mk_ipc_call`,
`mk_service_lookup`, plus `mk_yield` and `mk_exit`). `ip_proofs` and
`net_proofs` check the behavior end to end. see
[the network stack](../../docs/handbook/network/stack.md).
