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
| `OP_TX_PACKET` | transmit one Ethernet frame | status word |
| `OP_RX_PACKET` | poll one received frame | length plus frame bytes |
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

Frames live only in RX/TX DMA buffers and IPC payloads. The capsule does not
persist traffic, track application identity, keep captures, or log packet
payloads.

## Runtime lifecycle

The capsule claims the virtio NIC, maps its registers, turns INTx off,
allocates RX/TX virtqueues, draws a station address, enables queues, and serves
raw-frame IPC. Teardown releases DMA, register, and claim grants.

A transitional NIC that still has its legacy I/O BAR is driven over the legacy
registers. A modern-only one (what QEMU builds once the device sits behind its
IOMMU) is driven over the virtio 1.0 structures its vendor capabilities place in
a memory BAR, through the shared transport in `userland/nonos_virtio`, with
VERSION_1 and ACCESS_PLATFORM negotiated and a 12-byte net header.

## Failure model

Setup failure rolls back all earlier grants: a failed attempt releases the
claim, which takes every grant with it. Bring-up is tried a bounded number of
times with a sleep between tries (`nonos_libc::bring_up`); the capsule exits 2
when no NIC is present and 6 when one is present but never comes up. TX failure
and device faults are returned as protocol errors. Empty RX is non-fatal. Protocol parsing never
enters this driver.

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
  legacy I/O BAR): the log shows `[net-setup] transport` and then the legacy
  `regmap`, `irq`, `dma`, `negotiate`, `queues`, `done`; net.core's link
  probe answers up; DHCP completes; TX and RX hold under load.
- With the IOMMU lane (`-device intel-iommu` with `QEMU_IOMMU_OPTS`, and the
  device with `QEMU_IOMMU_VIRTIO`, i.e. `iommu_platform=on,disable-legacy=on`;
  see mk/10-qemu.mk) (modern-only 0x1041): the log shows `modern pci`, `modern
  regmap`, `dma`, `negotiate`, `queues`, `done`, with no MkMmioMap refusal of
  BAR1 and no claim and release loop; VERSION_1 and ACCESS_PLATFORM are
  negotiated; a ping round-trips and ARP replies parse (the 12-byte header is
  cut right); the VT-d log shows no DMA fault.
- A NIC that cannot come up: at most seven attempts over about six seconds,
  one `driver.virtio_net0: device present, bring-up failed` line, exit 6, no
  core left pegged. No virtio-net at all: exit 2 at once.
- A new MAC every boot. The driver never takes VIRTIO_NET_F_MAC and draws a
  locally administered address with `crypto_random` (`src/setup/station.rs`),
  so it needs the Crypto capability, which its manifest holds (0x1B8038). Confirm the address differs across two
  boots and that DHCP completes. QEMU's virtio-net accepts every frame by
  default; a hypervisor that filters frames by the MAC it assigned would drop
  the traffic, and that host needs its filter relaxed.
