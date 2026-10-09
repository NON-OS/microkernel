# capsule_net_l2

## Role

`capsule_net_l2` is the ethernet and ARP layer of the split network stack. it
sits above one wired NIC driver capsule and below `net.ip` and
`net.dhcp.client`. it frames and unframes ethernet, reports the card's MAC and
link state, resolves IPv4 next hops with ARP, and keeps the neighbour cache. it
holds no hardware authority of its own: the NIC driver capsule owns the card.

```text
net.ip / net.dhcp.client
    |
    | net.l2 requests over MkIpc (20-byte "NL2\0" envelope)
    v
net.l2 -- ARP cache + ethernet framing --> driver.virtio_net0 | e1000_0 | rtl8169_0 | rtl8139_0
                                           (MkIpcCall to the NIC capsule)
```

the desktop image does not carry it: there `net.core` holds the whole stack in
one capsule. it is built into the `microkernel-net-l2` through
`microkernel-net-ntp` profiles and the `microkernel-input-e2e-ps2` test image.

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x0001c`: Network (`0x04`), IPC (`0x08`) and Memory
  (`0x10`). the kernel mirror (`src/userspace/capsule_net_l2/spawn.rs`) requests
  exactly `Capability::IPC | Capability::Memory | Capability::Network`. no
  optional caps.
- service `service:4400:net.l2`, reply `reply:4401:endpoint.net.l2.reply`,
  declared for the manifest; the kernel registers the `net.l2` name at spawn.
- the feature is `nonos-capsule-net-l2`, namespace
  `systems.nonos.net.l2`.
- it reaches the kernel only over the `MkIpc*` syscall surface. it serves
  requests with `MkIpcRecvFrom` (`mk_ipc_recv_from`) and answers with
  `MkIpcReply` (`mk_ipc_reply`); it finds and drives the NIC driver capsule with
  `MkServiceLookup` (`mk_service_lookup`) and `MkIpcCall` (`mk_ipc_call`). it
  paces its bring-up loop with `MkYield` (`mk_yield`) and leaves with `MkExit`
  (`mk_exit`) only when the heap fails to initialise.
- the kernel `HELD` list lets only `net.core` and `net.l2` reach a wired card's
  driver (`src/services/registry/held.rs`).

## Interface contract

requests carry the 20-byte header shared by the stack capsules, magic
`0x4E4C3200` ("NL2\0"), version 1 (`src/protocol/header.rs`). the op set
(`src/protocol/ops.rs`):

| Op | Value | Who may call | Meaning |
|---|---|---|---|
| `OP_HEALTHCHECK` | 1 | anyone | liveness |
| `OP_GET_MAC` | 2 | anyone | the card's MAC |
| `OP_GET_LINK` | 3 | anyone | link state from the driver |
| `OP_SEND_FRAME` | 4 | `net.ip`, `net.dhcp.client` | send one ethernet frame |
| `OP_POLL_FRAME` | 5 | `net.ip`, `net.dhcp.client` | take one received frame |
| `OP_ARP_RESOLVE` | 6 | anyone | IPv4 to MAC, from the cache or by asking |
| `OP_SET_IP` | 7 | `net.dhcp.client` | the address ARP answers for |

`src/server/authz.rs` compares the sender's pid with the pid the registry bound
to the named service. the errnos are in `src/protocol/errno.rs`: `E_NO_LINK`,
`E_NO_NEIGHBOUR`, `E_TX_BUSY`, `E_RX_EMPTY`, `E_PERM` and the header errors.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x0001c` is the whole authority the capsule asks for:
Network, IPC and Memory. Network because the kernel registers a network service
only for a holder of it. no CoreExec, Crypto, FileSystem, Debug, Driver,
DeviceEnum, MMIO, IRQ, DMA or PIO: the NIC driver owns the hardware, and this
capsule reaches it only by IPC. the kernel installs the mask from the verified
manifest at spawn and the capsule cannot widen it.

## Privacy and persistence

the capsule sees link-layer addresses and frames in transit. it keeps the ARP
cache in memory and writes nothing to storage. it reads no user data and holds
no secret.

## Runtime lifecycle

`_start` (`src/main.rs`) initialises the heap, then loops `setup::run` behind
`MkYield` until a NIC is found. `first_available` (`src/setup/discover.rs`)
binds the first of `NIC_CANDIDATES` the registry knows, probed by
`MkServiceLookup`: `driver.virtio_net0`, `driver.e1000_0`, `driver.rtl8169_0`,
`driver.rtl8139_0`. it binds no Wi-Fi card. once bound it serves inbox 0 forever
(`src/server/runner.rs`), one request at a time. it is not restarted and the NIC
binding does not move during a boot.

## Failure model

a request that fails header parsing is still answered, under the op and request
id it names, or under zeros when it is too short (`refused` in
`src/protocol/decode.rs`). an unknown op gets `E_BAD_OP`. a caller that is not
the bound owner of the named service gets `E_PERM`. with no link the data-plane
ops return `E_NO_LINK`; a poll with nothing waiting returns `E_RX_EMPTY`; an ARP
miss returns `E_NO_NEIGHBOUR` after a broadcast request goes out, so the caller
asks again. if the heap cannot initialise at start, `_start` calls
`mk_exit(1)` and the capsule does not register.

## Current implemented surface

the seven ops above, each with a handler in `src/server/handlers`. ARP
(`src/arp/handle.rs`) ignores any operation but request and reply and any
sender whose MAC is zero or a group address; it learns a sender only when the
address is one a neighbour can hold (not 0/8, loopback, 224 and up, or our own),
only when the binding was solicited, and never rebinds a known address to a new
MAC; it answers a request for the local address once `OP_SET_IP` has set one.
the cache holds `ENTRY_CAP`, 64 entries, and evicts the oldest.

## Wire format

the 20-byte v1 envelope (`src/protocol/header.rs`): `u32` magic
(`0x4E4C3200`), `u16` version (1), `u16` op, `u16` flags (unused on a request,
errno on a response), `u16` reserved, `u32` request_id, `u32` payload_len, then
the payload. `IPC_PAYLOAD_MAX` is `ETH_FRAME_MAX + 64`, the 1514-byte max
ethernet frame plus a 64-byte margin (`src/protocol/limits.rs`), so a caller can
wrap one full-MTU frame per message without splitting. downstream, the NIC
driver capsule speaks its own 20-byte "NNET" v1 envelope
(`src/nic_client/wire.rs`), reached with `MkIpcCall`.

## State ownership

the capsule owns the bound NIC port and pid, the local MAC, the configured IPv4
address for ARP, and the ARP neighbour cache (`src/state.rs`,
`src/arp/cache`). all of it lives in memory for the life of the process. it owns
no packet buffers beyond the single request and reply slabs the runner holds.

## Operating rules

- one NIC per boot. it binds the first candidate found and does not move to
  another.
- it reads the card only when a caller polls with `OP_POLL_FRAME`: nothing is
  received while no one asks.
- only `net.ip` and `net.dhcp.client` may send or poll frames; only
  `net.dhcp.client` may set the local address.
- adding a NIC class is one line in `NIC_CANDIDATES`; the data plane upstream
  sees no difference.

## Release target

0.9.2.

## Release evidence

`userland/l2_proofs` runs the request header decode and the reply a refused
frame gets. `userland/net_proofs` runs the ARP parser and the learning rules.
the `microkernel-net-l2` through `microkernel-net-ntp` profiles and the
`microkernel-input-e2e-ps2` image boot the capsule against a real NIC driver
capsule.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x0001c`, kernel mirror requests the
      same three caps.
- [ ] the seven ops answer, and authz refuses `OP_SEND_FRAME`, `OP_POLL_FRAME`
      and `OP_SET_IP` to a sender that is not the bound owner.
- [ ] ARP learns only solicited, neighbour-valid bindings and never rebinds.
- [ ] `l2_proofs` and `net_proofs` pass.

## Explicit non-goals today

- no IP, routing, TCP, UDP, DHCP or DNS: those are the capsules above it.
- no Wi-Fi: only wired cards are candidates.
- no second NIC and no failover during a boot.
- no socket policy and no hardware access of its own.

## Verification

the magic, version and 20-byte layout are fixed in `src/protocol/header.rs`; the
op set in `src/protocol/ops.rs`; the cap mask in `Capsule.mk` and
`src/userspace/capsule_net_l2/spawn.rs`. the `MkIpc*` calls are the only kernel
surface the capsule touches (`mk_ipc_recv_from`, `mk_ipc_reply`, `mk_ipc_call`,
`mk_service_lookup`, plus `mk_yield` and `mk_exit`). `l2_proofs` and
`net_proofs` check the behavior end to end. see
[the network stack](../../docs/handbook/network/stack.md).
