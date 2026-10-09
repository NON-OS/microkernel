# capsule_driver_e1000e

Status: not in the 0.9.2 image. The capsule builds and its host proofs
pass, but it has no owner certificate and the kernel never spawns it:
it is not yet wired into the kernel mirror and the stack.

## Role

`capsule_driver_e1000e` is the Intel e1000e Ethernet driver capsule: the
82574 and 82583, and the I217, I218 and I219 PHYs behind the MAC in Intel
PCH chipsets (Linux boards pch_lpt to pch_ptp). It owns the PCI function,
BAR0, the RX/TX DMA rings and raw Ethernet frame movement, and stops at the
frame boundary. The chip list, Linux sources compared and what is verified
are in `docs/hardware/e1000e.md`.

```text
net.core / net.l2
    |
    | raw Ethernet IPC (NNET, as driver.e1000_0)
    v
driver.e1000e_0 -- broker MMIO + DMA rings --> 82574 / I217 / I218 / I219
    |
    `-- polled: no interrupt is bound
```

## Microkernel contract

- `MkDeviceList` finds the first Intel Ethernet function whose ID is in
  `src/constants/ids.rs`.
- `MkDeviceClaim` claims it; `MkPciConfigWrite` turns on Memory Space and
  Bus Master Enable; `MkPciConfigRead` reads the I219 descriptor ring status
  word (0xE4) before a reset.
- `MkMmioMap` maps BAR0; `MkDmaMap` takes the RX ring, RX buffers, TX ring
  and TX buffers.
- `MkIpcRecv` and `MkIpcSend` serve `service:4270:driver.e1000e_0`.
- `CryptoRandom` draws the station address each boot.

No interrupt is bound and IMS is never written.

## Interface contract

Same operations as `driver.e1000_0`: `OP_HEALTHCHECK`, `OP_LINK_STATUS`
(1 byte), `OP_MAC_ADDRESS` (6 bytes), `OP_TX_PACKET`, `OP_RX_PACKET`
(length and frame), `OP_STATS` (48-byte snapshot).

## Authority

`CAPSULE_REQUIRED_CAPS = 0xB8038`: IPC, Memory, Crypto, Driver,
DeviceEnum, Mmio, Dma. `CAPSULE_OPTIONAL_CAPS = 0x100` (Debug) is granted
only by a `capsule-serial-debug` build; without it the `e1000e:` lines are
dropped. No Irq, Network, filesystem or admin authority.

## Privacy and persistence

Frames live in DMA buffers and IPC payloads only while they are handled.
The factory address is never used: a locally administered address is drawn
each boot, and bring-up fails closed without entropy. Nothing is stored.

## Runtime lifecycle

Discover, claim, enable bus mastering, map, take DMA grants, then bring the
part up in Linux's order (quiesce, PCH PHY workarounds, global reset,
hardware bits, address, link, rings) and serve IPC until the process ends.

## Failure model

No device: one line and `EXIT_ABSENT`. A failing step gives back every
grant, logs `e1000e: <step>`, and the attempt is retried on the shared
bounded schedule; running out is `EXIT_GAVE_UP`. Link down and RX empty are
answers, not errors.

## Current implemented surface

Device table for 64 IDs, polled RX/TX on legacy descriptors with 2048-byte
buffers, CRC stripped, short frames padded, ULP exit, SMBus to PCIe switch,
LANPHYPC toggle, I219 ring flush, K1 timing on pch_mtp and later, link
state lines.

## Wire format

As `capsule_driver_e1000`: 20-byte NNET header, 4-byte status, per-op data.
The twelfth stats word is CTRL_EXT here (e1000 sends zero).

## State ownership

The capsule owns the rings, buffers, register state and station address;
`net.core` and `net.l2` own protocol interpretation; the kernel owns grants.

## Operating rules

Frames only; no ARP, IP or socket logic here. Every wait is on the uptime
clock. Every step that can fail logs one `e1000e:` line.

## Release target

A signed, spawned raw-frame NIC service for the 82574 (QEMU) and the
I217/I218/I219 laptops and desktops of 2013 onward.

## Release evidence

A real boot on at least one I219 machine with `log e1000e` showing the `up`
line and a `link up` line, and DHCP completing through `net.core`.

## Release checklist

- Wired into the kernel mirror and the stack, sealed by the owner.
- QEMU `-device e1000e` frame round trip.
- One I219 boot photographed with the lines in `docs/hardware/e1000e.md`.

## Explicit non-goals today

No interrupts, MSI-X, jumbo frames, flow control, EEE, Wake-on-LAN, VLAN,
offloads, RSS, timestamping, or NVM access.

## Verification

- Build: `cargo build --release --target ../x86_64-nonos-user.json
  -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem`
- Proofs: `cd userland/e1000e_proofs && cargo test`
- Not verified on any hardware or in QEMU yet.
