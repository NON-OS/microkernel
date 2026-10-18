# capsule_driver_virtio_net

## Role

`capsule_driver_virtio_net` is the virtio network driver capsule. It sends and
receives raw Ethernet frames and exposes the frame service to userland network
capsules. It does not contain ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing,
or firewall policy.

```text
net.core / net.l2
    |
    | raw frame IPC
    v
driver.virtio_net0 -- virtqueue DMA --> virtio-net device
```

## Microkernel contract

The capsule uses only Mk/broker interfaces:

- `MkDeviceList` locates the virtio network device.
- `MkDeviceClaim` owns the device claim.
- `MkPioGrant` takes a transitional device's legacy I/O BAR, or `MkMmioMap`
  maps the register window (legacy MMIO, or the modern structures).
- Completions are polled: `setup::irq` turns the legacy INTx line off and no
  interrupt is bound, so the manifest holds no `Irq`.
- `MkDmaMap` and `MkDmaUnmap` allocate RX/TX rings and packet buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.virtio_net0` on
  `service:4204:driver.virtio_net0`.

The kernel remains mechanism only: capability checks, address spaces, IPC, and
grant revocation. Network protocol logic belongs to userland network capsules.
Only `net.core` and `net.l2` may send to `driver.virtio_net0`: the kernel holds
the endpoint to them (`src/services/registry/held.rs`), by name and by pid.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_LINK_STATUS` | link-up state | 1 byte |
| `OP_MAC_ADDRESS` | the station address drawn this boot | 6 bytes |
| `OP_TX_PACKET` | queue one Ethernet frame, 14 to 1514 bytes; `E_AGAIN` while every transmit buffer is still the device's | status word |
| `OP_RX_PACKET` | poll one received frame, at most 1514 bytes | length plus frame bytes |
| `OP_RX_BATCH` | every received frame waiting, in one reply; a request carrying a u32 batch number takes frames, and the same number asks again for a lost reply | frames |

## Authority

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`,
`Mmio`, `Dma` and `Pio` (`CAPSULE_REQUIRED_CAPS = 0x1B8038`); the driver polls, so no `Irq`.
`Debug` is optional (`CAPSULE_OPTIONAL_CAPS = 0x100`): only a `capsule-serial-debug` build grants it.
Crypto is for drawing a per-boot station address with `crypto_random`. It has no socket, route table,
firewall, DNS, DHCP, filesystem, or admin authority.

```text
allowed:   virtio NIC claim, MMIO or PIO registers, DMA rings, kernel randomness, raw-frame IPC
forbidden: IP stack, sockets, packet store, routing policy, kernel drivers
```

## Privacy and persistence

Frames live only in RX/TX DMA buffers and IPC payloads, and only until they
are delivered: a receive buffer is wiped before it goes back to the device, a
transmit buffer at the first send or receive poll (net.core polls every
cycle) after the device returns it, and the request, reply and batch buffers
once the frame has gone on (plan/50 C8). The capsule does not persist
traffic, track application identity, keep captures, or log packet payloads or
its station address.

## Runtime lifecycle

The capsule claims the virtio NIC, maps its registers, turns INTx off, resets
the device (status 0, read back by the clock, on either transport), negotiates, draws a station address, maps the RX/TX virtqueues,
enables the queues, only then turns Bus Master on, and serves
raw-frame IPC. Transmit never waits: a frame is queued and answered, and
buffers come back, wiped, at the next send or receive poll. Teardown resets the device, then releases
the claim; the broker clears Bus Master before it frees any grant, and the
driver unmaps nothing itself.

Whenever the function presents the virtio 1.0 structures (a transitional NIC
does too), it is driven over them, through the shared transport in
`userland/nonos_virtio`, with VERSION_1 negotiated, ACCESS_PLATFORM whenever the
device offers it, and a 12-byte net header. A transitional function keeps its
legacy registers when it lacks a usable structure; a modern-only one is refused. Both transports take a 256-entry ring
and nothing else: the grant and the ring offsets are sized for it, and a legacy
device cannot be told a smaller ring, so any other QueueNumMax is refused.

## Failure model

Setup failure releases the claim, which takes every grant with it. Bring-up is
tried a bounded number of times with a sleep between tries
(`nonos_libc::bring_up`); the capsule exits 2 when no NIC is present and 6 when
one is present but never comes up. A received frame longer than 1514 bytes, or
an entry naming no buffer, is dropped and its buffer recycled. A transmit ring
that stays full for two seconds with the link up is a stopped device: the
capsule logs `transmit stalled`, resets and releases it, and brings it up again
with the same station address (init restarts no driver); one that stalls
again without completing a frame is reset after a wait that doubles, up to a
minute, and logged once per step. Every wait is by the uptime clock. Empty RX
is non-fatal. Protocol parsing never enters this driver.

## Current implemented surface

- Legacy and virtio 1.0 (modern) PCI transports, chosen per function.
- Initializes RX/TX queues.
- Validates queue physical addresses.
- Draws a station address per bring-up and reports it; the device's MAC is never taken.
- Serves link, MAC, RX, RX batch and TX operations over IPC.
- Keeps protocol parsing above the driver boundary.

## Wire format

Requests use the `NNET` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. MAC replies carry
6 bytes. Link replies carry 1 byte. RX replies carry length plus frame bytes.
TX requests carry one Ethernet frame bounded by `MAX_ETHERNET_FRAME`.

## State ownership

The capsule owns RX/TX virtqueues, packet buffers, the register grant, the
station address, and link state. `net.core` and `net.l2` own Ethernet/ARP
behavior. The kernel owns no
packet buffer beyond broker mappings.

## Operating rules

- Keep the driver protocol-blind.
- Never persist frames or peer identity.
- Treat RX-empty and link-down as ordinary states.
- Keep all hardware access behind broker grants.

## Release target

The finished virtio-net capsule is a signed raw-frame NIC service with stable
RX/TX queue refill, interrupt recovery, link/MAC reporting, QEMU validation, and
delivery into `net.core`. It remains protocol-blind: no ARP, IP, sockets,
firewall, or packet history is allowed in the driver.

## Release evidence

Release requires QEMU `virtio-net` frame round trip, `net.l2` ARP proof,
teardown DMA revocation proof, and host-network validation with packet boundaries
intact.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU virtio-net frame round trip passes.
- `net.l2` ARP proof passes above this driver.
- Teardown proof shows DMA/register/device claim revocation.
- Static gate proves no kernel network stack is imported.

## Explicit non-goals today

No ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing, firewall, NAT, packet
capture, or traffic analytics live in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-virtio-net`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: virtio-net must stay broker-only and must not import
  kernel network or driver code.
- Documentation check: this README is part of the driver-capsule acceptance
  criteria.
- Handbook: [drivers](../../docs/handbook/drivers.md).

## Real bring-up checklist

Proved on the host, without a device: the transport choice, the capability
parse, feature negotiation, queue programming and the doorbell arithmetic
(`userland/virtio_transport_proofs`), and the receive path, the per-transport
header length and the modern MAC read (`userland/virtio_net_proofs`). Only a
boot shows the rest. Under QEMU q35, confirm both ways:

- Without an IOMMU (`-device virtio-net-pci`, transitional 0x1000 with its
  legacy I/O BAR and the virtio 1.0 structures): the log shows
  `[net-setup] transport` and then `modern pci`, `modern regmap`,
  `negotiate`, `dma`, `queues`, `done`; net.core's link probe answers up;
  DHCP completes; TX and RX hold under load. With `disable-modern=on` the
  legacy path runs instead: `regmap`, `irq`, `negotiate`, `dma`, `queues`,
  `done`.
- With the IOMMU lane (`-device intel-iommu` with `QEMU_IOMMU_OPTS`, and the
  device with `QEMU_IOMMU_VIRTIO`, i.e. `iommu_platform=on,disable-legacy=on`;
  see mk/10-qemu.mk) (modern-only 0x1041): the log shows `modern pci`, `modern
  regmap`, `negotiate`, `dma`, `queues`, `done`, with no MkMmioMap refusal of
  BAR1 and no claim and release loop; VERSION_1 and ACCESS_PLATFORM are
  negotiated; a ping round-trips and ARP replies parse (the 12-byte header is
  cut right); the VT-d log shows no DMA fault.
- A NIC whose legacy QueueNumMax is not 256 (bhyve's 1024, or QEMU with
  `rx_queue_size=1024` on a device without the modern structures): refused
  with `legacy queue size is not 256`, the device marked FAILED, no ring
  address written.
- A NIC that cannot come up: at most seven attempts over about six seconds,
  one `driver.virtio_net0: device present, bring-up failed` line, exit 6, no
  core left pegged. No virtio-net at all: exit 2 at once.
- A new MAC every boot. The driver never takes VIRTIO_NET_F_MAC and draws a
  locally administered address with `crypto_random` (`src/setup/station.rs`),
  so it needs the Crypto capability, which its manifest holds (0x1B8038). Confirm the address differs across two
  boots and that DHCP completes. QEMU's virtio-net accepts every frame by
  default; a hypervisor that filters frames by the MAC it assigned would drop
  the traffic, and that host needs its filter relaxed.
