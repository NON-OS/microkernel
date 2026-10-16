# capsule_driver_igb

Status: wired, not yet signed. The kernel mirror (`src/hardware/igb_capsule`)
embeds and spawns it under `nonos-capsule-driver-igb` (the
`microkernel-driver-igb` build), and net.core and net.l2 reach it through
`src/services/registry/nics.rs`. It has no owner certificate yet, so no
image carries it until the owner signs it.

## Role

`capsule_driver_igb` is the Intel 82575, 82576, 82580, I350, I210 and I211
GbE Ethernet driver capsule, for a copper port. It owns the PCI
function, BAR0, one advanced RX queue, one advanced TX queue and raw
Ethernet frame movement, and stops at the frame boundary. QEMU's `igb`
device is an 82576 and drives this path.

```text
net.core / net.l2
    |
    | raw Ethernet IPC (NNET, as driver.e1000_0)
    v
driver.igb_0 -- broker MMIO + DMA rings --> 82575 .. I211 (internal PHY)
    |
    `-- polled: no interrupt is bound
```

## Microkernel contract

- `MkDeviceList` finds the first Intel Ethernet function whose ID is in
  `facts/ids.rs` (as observed, unverified).
- `MkDeviceClaim` claims it; `MkPciConfigWrite` turns on Memory Space and
  Bus Master Enable.
- `MkMmioMap` maps BAR0; `MkDmaMap` takes the RX ring, RX buffers, TX ring
  and TX buffers.
- `MkIpcRecv` and `MkIpcSend` serve `service:4242:driver.igb_0`.
- `CryptoRandom` draws the station address each boot.

No interrupt is bound and IMS is never written.

## Interface contract

Same operations as `driver.e1000_0`: `OP_HEALTHCHECK`, `OP_LINK_STATUS`
(1 byte), `OP_MAC_ADDRESS` (6 bytes), `OP_TX_PACKET`, `OP_RX_PACKET`
(length and frame), `OP_STATS` on 7 (48-byte snapshot). 6, net.core's
OP_RX_BATCH, is refused.

## Authority

`CAPSULE_REQUIRED_CAPS = 0xB8038`: IPC, Memory, Crypto, Driver,
DeviceEnum, Mmio, Dma. `CAPSULE_OPTIONAL_CAPS = 0x100` (Debug) is granted
only by a `capsule-serial-debug` build; without it the `igb:` lines are
dropped. No Irq, Network, filesystem or admin authority.

## Privacy and persistence

Frames live in DMA buffers and IPC payloads only while they are handled.
The factory address is never used: a locally administered address is drawn
each boot, before either enable bit is written, and bring-up fails closed
without entropy. Nothing is stored.

## Runtime lifecycle

Discover, claim, enable bus mastering, map, take DMA grants, then bring the
part up stage by stage (`bring/stage.rs`: PCIe master stop, reset, media check,
address and filters, link with Go Link Disconnect cleared, PHY power-up
under this port's SW/FW semaphore, MAC enables, TX queue, RX queue) and
serve IPC until the process ends.

## Failure model

No device: one line and `EXIT_ABSENT`. A failing step stands the part
down, gives back every grant, logs `igb: <step>`, and the attempt is
retried on the shared bounded schedule; running out is `EXIT_GAVE_UP`.
A fiber, SerDes, SGMII or backplane port is refused by name (CTRL_EXT
link mode), as is an 82575 with no NVM. Link down and RX empty are
answers, not errors.

## Current implemented surface

Device table for 35 IDs over seven MACs, polled RX/TX on advanced
one-buffer descriptors with 2048-byte buffers, CRC stripped, short frames
padded, PHY power-up over MDIC (paged on the I210/I211), autonegotiation
restarted, link state lines.

## Wire format

As `capsule_driver_e1000`: 20-byte NNET header, 4-byte status, per-op data.

## State ownership

The capsule owns the rings, buffers, register state and station address;
`net.core` and `net.l2` own protocol interpretation; the kernel owns grants.

## Operating rules

Frames only; no ARP, IP or socket logic here. Every wait is on the uptime
clock. Every step that can fail logs one `igb:` line.

## Release target

A signed, spawned raw-frame NIC service for the igb copper ports on
servers, workstations and boards from 2008 onward (82576, I350, I210,
I211), and for QEMU's `igb` device.

## Release evidence

A boot under QEMU with `-device igb` and a real boot on one I210, I211 or
I350 machine, each with `log igb` showing the `up` line and a `link up`
line, and DHCP completing through `net.core`.

## Release checklist

- Signed by the owner and added to the desktop image's features.
- The QEMU `igb` lane and one machine photographed with those lines.

## Explicit non-goals today

No interrupts, MSI-X, multiple queues, VMDq or SR-IOV, jumbo frames, flow
control, EEE, Wake-on-LAN, VLAN, offloads, RSS, timestamping, or NVM
access. No fiber, SerDes, SGMII or backplane port. Only the first igb
function in the device list is taken.

## Verification

- Build: `cargo build --release --target ../x86_64-nonos-user.json
  -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem`
- Proofs: `cd userland/igb_proofs && cargo test`
- Not verified on any hardware or under QEMU yet.
