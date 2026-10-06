# capsule_net_l2

## Role

`capsule_net_l2` is the Ethernet and ARP layer of the split network stack. It
sits above one wired NIC driver capsule and below `net.ip` and
`net.dhcp.client`. It frames and unframes Ethernet, reports the card's MAC and
link, resolves IPv4 next hops with ARP, and keeps the neighbour cache.

```text
net.ip / net.dhcp.client
    |
    | NL2 requests over IPC
    v
net.l2 -- ARP cache + Ethernet framing --> driver.virtio_net0 | e1000_0 | rtl8169_0 | rtl8139_0
```

The desktop image does not carry it: there `net.core` holds the whole stack.
It is built into the `microkernel-net-l2` through `microkernel-net-ntp`
profiles and the `microkernel-input-e2e-ps2` test image.

## Service and endpoints

- Handle `net.l2`, service endpoint `service:4400:net.l2`, reply endpoint
  `reply:4401:endpoint.net.l2.reply`.
- Kernel mirror `src/userspace/capsule_net_l2`.
- `_start` retries `setup::run` until a NIC is found, then serves inbox 0.
  `first_available` binds the first of `NIC_CANDIDATES` the registry knows:
  `driver.virtio_net0`, `driver.e1000_0`, `driver.rtl8169_0`,
  `driver.rtl8139_0`. It binds no Wi-Fi card.

## Interface

Requests carry the 20 byte header shared by the stack capsules, with magic
`0x4E4C3200` ("NL2\0") and version 1 (`src/protocol/header.rs`).

| Op | Value | Who may call | Meaning |
|---|---|---|---|
| `OP_HEALTHCHECK` | 1 | anyone | liveness |
| `OP_GET_MAC` | 2 | anyone | the card's MAC |
| `OP_GET_LINK` | 3 | anyone | link state from the driver |
| `OP_SEND_FRAME` | 4 | `net.ip`, `net.dhcp.client` | send one Ethernet frame |
| `OP_POLL_FRAME` | 5 | `net.ip`, `net.dhcp.client` | take one received frame |
| `OP_ARP_RESOLVE` | 6 | anyone | IPv4 to MAC, from the cache or by asking |
| `OP_SET_IP` | 7 | `net.dhcp.client` | the address ARP answers for |

`src/server/authz.rs` compares the sender's pid with the pid the registry
bound to the named service. A frame that fails header parsing is still
answered, under the op and request id it names, or under zeros when it is too
short (`refused` in `src/protocol/decode.rs`). The errnos are in
`src/protocol/errno.rs`: `E_NO_LINK`, `E_NO_NEIGHBOUR`, `E_TX_BUSY`,
`E_RX_EMPTY`, `E_PERM` and the header errors.

## ARP

`OP_ARP_RESOLVE` answers from the cache, or broadcasts a request and returns
`E_NO_NEIGHBOUR` so the caller asks again. `on_inbound` (`src/arp/handle.rs`):

- ignores any operation but request and reply, and any sender whose MAC is
  zero or a group address (`is_station`);
- learns a sender only when its address is one a neighbour can hold
  (`is_neighbour_ip`: not 0/8, loopback, 224 and up, or our own address), only
  when the binding was solicited, and never rebinds a known address to a new
  MAC;
- answers a request for the local address once `OP_SET_IP` has set one.

The cache holds `ENTRY_CAP`, 64 entries, and evicts the oldest.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
(`0x10`). Network because the kernel registers a network service only for a
holder of it. No CoreExec, Crypto, FileSystem, Debug, Driver, DeviceEnum,
MMIO, IRQ, DMA or PIO: the NIC driver owns the hardware. The kernel's `HELD`
list lets only `net.core` and `net.l2` reach a wired card's driver
(`src/services/registry/held.rs`).

## Privacy and persistence

The capsule sees link-layer addresses and frames in transit. It keeps the ARP
cache in memory and writes nothing to storage.

## What it does not do

- No IP, routing, TCP, UDP, DHCP or DNS: those are the capsules above it.
- No Wi-Fi: only wired cards are candidates.
- One NIC per boot. It binds the first candidate found and does not move to
  another.
- It reads the card only when a caller polls: nothing is received while no one
  asks.

## Build and verify

- `make nonos-mk-net-l2`, then `nonos-mk-net-l2-sign` and
  `nonos-mk-net-l2-verify` (rules from `nonos-mk/capsule.mk`).
- `userland/l2_proofs` runs the request header decode and the reply a refused
  frame gets. `userland/net_proofs` runs the ARP parser and the learning rules.

See [the network stack](../../docs/handbook/network/stack.md).
