# capsule_net_ip

## Role

`capsule_net_ip` is the IPv4 layer of the split network stack. It owns the
interface address and the route table, builds outbound IPv4 packets and hands
them to `net.l2`, and checks inbound packets before passing them up to
`net.udp` and `net.tcp`. It answers ICMP echo requests itself.

```text
net.udp / net.tcp
    |
    | NIP4 requests over IPC
    v
net.ip -- IPv4 parse/build + route table + ICMP echo --> net.l2
```

The desktop image does not carry it: there `net.core` registers the `net.ip`
name itself and answers ICMP only. It is built into the `microkernel-net-ip`
through `microkernel-net-ntp` profiles and the `microkernel-input-e2e-ps2`
test image.

## Service and endpoints

- Handle `net.ip`, service endpoint `service:4402:net.ip`, reply endpoint
  `reply:4403:endpoint.net.ip.reply`.
- Kernel mirror `src/userspace/capsule_net_ip`.
- It waits until `net.l2` is registered, then serves inbox 0.

## Interface

Requests carry the 20 byte header shared by the stack capsules, with magic
`0x4E495034` ("NIP4") and version 1 (`src/protocol/header.rs`).

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
to the named service. No capsule registers `net.admin`, so `OP_ROUTE_ADD` and
`OP_ROUTE_CLEAR` answer `E_PERM` to everyone. A frame that fails header parsing
is still answered, under the op and request id it names, or under zeros when it
is too short.

## Packets

- Egress: `send` refuses with no address, takes the next hop from the longest
  matching route (prefix 0 is the default), resolves it through `net.l2` and
  sends the frame. The table holds `TABLE_CAP`, 16 routes.
- Ingress happens inside `OP_POLL_PACKET`: up to `POLL_BUDGET`, eight, frames
  are pulled from `net.l2` per call. `parse` rejects a bad version, length or
  checksum and any fragment; options are carried but not decoded.
  `from_frame` drops a packet whose source no host can have (0/8, loopback,
  224 and up, or our own address) and one addressed to someone else. A
  malformed frame is dropped and the poll goes on.
- An echo request to our address is answered by `try_reply` and never reaches
  a caller. One sent to the broadcast address is not answered.
- The `tcp-chaos` cargo feature, off in every shipped build, drops inbound TCP
  segments 3 and 6 to exercise retransmission.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
(`0x10`). Network because the kernel registers a network service only for a
holder of it. No CoreExec, Crypto, FileSystem, Debug or hardware bits.

## Privacy and persistence

The capsule holds the interface address, prefix, gateway and route table in
memory. Packets pass through and are not kept or logged.

## What it does not do

- No IPv6, no fragmentation or reassembly, no forwarding between interfaces.
- The route table changes only through `OP_SET_CONFIG`, since nothing holds
  `net.admin`.
- It reads `net.l2` only when a caller polls.

## Build and verify

- `make nonos-mk-net-ip`, then `nonos-mk-net-ip-sign` and
  `nonos-mk-net-ip-verify`.
- `userland/ip_proofs` drives the real ingress and ICMP echo path against
  frames played from the host. `userland/net_proofs` runs the IPv4 and ICMP
  parsers.

See [the network stack](../../docs/handbook/network/stack.md).
