# capsule_driver_igc

Status: not in the 0.9.2 image. The capsule builds and its host proofs
pass, but it has no owner certificate and the kernel never spawns it:
it is not yet wired into the kernel mirror and the stack.

## Role

`capsule_driver_igc` is the Intel I225 and I226 2.5 GbE Ethernet driver
capsule (Linux `igc`). It owns the PCI function, BAR0, one advanced RX
queue, one advanced TX queue and raw Ethernet frame movement, and stops at
the frame boundary. The chip list, Linux sources compared and what is
verified are in `docs/hardware/igc.md`.

```text
net.core / net.l2
    |
    | raw Ethernet IPC (NNET, as driver.e1000_0)
    v
driver.igc_0 -- broker MMIO + DMA rings --> I225 / I226 (internal PHY)
    |
    `-- polled: no interrupt is bound
```

## Microkernel contract

- `MkDeviceList` finds the first Intel Ethernet function whose ID is in
  the igc table (every `IGC_DEV_ID_*` in Linux igc_hw.h).
- `MkDeviceClaim` claims it; `MkPciConfigWrite` turns on Memory Space and
  Bus Master Enable.
- `MkMmioMap` maps BAR0; `MkDmaMap` takes the RX ring, RX buffers, TX ring
  and TX buffers.
- `MkIpcRecv` and `MkIpcSend` serve `service:4272:driver.igc_0`.
- `CryptoRandom` draws the station address each boot.

No interrupt is bound and IMS is never written.

## Interface contract

Same operations as `driver.e1000_0`: `OP_HEALTHCHECK`, `OP_LINK_STATUS`
(1 byte), `OP_MAC_ADDRESS` (6 bytes), `OP_TX_PACKET`, `OP_RX_PACKET`
(length and frame), `OP_STATS` (48-byte snapshot).

## Authority

`CAPSULE_REQUIRED_CAPS = 0xB8038`: IPC, Memory, Crypto, Driver,
DeviceEnum, Mmio, Dma. `CAPSULE_OPTIONAL_CAPS = 0x100` (Debug) is granted
only by a `capsule-serial-debug` build; without it the `igc:` lines are
dropped. No Irq, Network, filesystem or admin authority.

## Privacy and persistence

Frames live in DMA buffers and IPC payloads only while they are handled.
The factory address is never used: a locally administered address is drawn
each boot, before either enable bit is written, and bring-up fails closed
without entropy. Nothing is stored.

## Runtime lifecycle

Discover, claim, enable bus mastering, map, take DMA grants, then bring the
part up in Linux's order (PCIe master stop, global reset, address and
filters, PHY power-up under the SW/FW semaphore, link, MAC enables, TX
queue, RX queue) and serve IPC until the process ends.

## Failure model

No device: one line and `EXIT_ABSENT`. A failing step stands the part
down, gives back every grant, logs `igc: <step>`, and the attempt is
retried on the shared bounded schedule; running out is `EXIT_GAVE_UP`.
Link down and RX empty are answers, not errors.

## Current implemented surface

Device table for 16 IDs, polled RX/TX on advanced one-buffer descriptors
with 2048-byte buffers, CRC stripped, short frames padded, PHY power-up
over MDIC, autonegotiation restarted, link state lines with 2500 decoded.

## Wire format

As `capsule_driver_e1000`: 20-byte NNET header, 4-byte status, per-op data.

## State ownership

The capsule owns the rings, buffers, register state and station address;
`net.core` and `net.l2` own protocol interpretation; the kernel owns grants.

## Operating rules

Frames only; no ARP, IP or socket logic here. Every wait is on the uptime
clock. Every step that can fail logs one `igc:` line.

## Release target

A signed, spawned raw-frame NIC service for the I225/I226 desktops and
laptops of 2019 onward.

## Release evidence

A real boot on one I225 or I226 machine with `log igc` showing the `up`
line and a `link up` line, and DHCP completing through `net.core`.

## Release checklist

- Wired into the kernel mirror and the stack, sealed by the owner.
- One I225/I226 boot photographed with the lines in `docs/hardware/igc.md`.

## Explicit non-goals today

No interrupts, MSI-X, multiple queues, jumbo frames, flow control, EEE,
Wake-on-LAN, VLAN, offloads, RSS, timestamping, or NVM access. Only the
first port of a multi-port board is taken.

## Verification

- Build: `cargo build --release --target ../x86_64-nonos-user.json
  -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem`
- Proofs: `cd userland/igc_proofs && cargo test`
- Not verified on any hardware. QEMU has no I225/I226 model.
