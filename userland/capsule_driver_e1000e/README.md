# capsule_driver_e1000e

Status: wired, not yet signed. The kernel mirror (`src/hardware/e1000e_capsule`)
embeds and spawns it under `nonos-capsule-driver-e1000e` (the
`microkernel-driver-e1000e` build), and net.core and net.l2 reach it through
`src/services/registry/nics.rs`. It has no owner certificate yet, so no image
carries it until the owner signs it.

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
- `MkDeviceClaim` claims it; `MkPciConfigWrite` turns on Memory Space with
  Bus Master off. Bus Master goes on only with both units stopped and both
  rings naming the driver's own memory: after the reset, and on an I219 that
  has run on those rings and asks for a descriptor flush, for that flush,
  off again before the reset lands. `MkPciConfigRead` reads the I219
  descriptor ring status word (0xE4) before a reset.
- `MkMmioMap` maps BAR0, which must reach `BAR0_MIN_BYTES` (0x6000) in the
  device record and as mapped; `MkDmaMap` takes the RX ring, RX buffers, TX
  ring and TX buffers.
- `MkDeviceRelease` gives the device back. It clears Bus Master and reads it
  back before it frees every grant, so the driver unmaps nothing by hand.
- `MkIpcRecv` and `MkIpcSend` serve `service:4270:driver.e1000e_0`.
- `CryptoRandom` draws the station address each boot.

No interrupt is bound and IMS is never written.

## Interface contract

Same operations as `driver.e1000_0`: `OP_HEALTHCHECK`, `OP_LINK_STATUS`
(1 byte), `OP_MAC_ADDRESS` (6 bytes), `OP_TX_PACKET`, `OP_RX_PACKET`
(length and frame), `OP_STATS` on 7 (48-byte snapshot).

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

Discover, claim with Bus Master off, map, take DMA grants, then bring the
part up in Linux's order (quiesce, PCH PHY workarounds, global reset,
hardware bits, address, link, rings), turn Bus Master on, enable both units
and serve IPC until the process ends.

An I219 whose status word asks for a descriptor flush at claim holds
descriptors fetched under a previous owner (a dead capsule, or firmware),
and they may name frames since freed. No datasheet in the tree says that
rewriting TDBAL/RDBAL drops them, so the driver fails closed: Bus Master
does not go on for a flush, and the part is reset with Bus Master off. A
reset that does not complete ends the attempt as `I219 descriptors pending,
a function reset is needed`, Bus Master never on, and the device goes back.
The kernel's K10a function level reset at claim clears those descriptors
wherever the function offers FLR, so the word should read clear by then.
The flush (Linux e1000_flush_desc_rings) runs only on descriptors the
driver's own rings gave the part: before a transmit-hang recovery's reset,
and at teardown.

A transmit watchdog runs on every send: frames in flight with the link up
and none completed for 5 s is a hung transmitter. The part is quiesced and
reset with Bus Master off, both rings restart from slot 0, and the part goes
back on the air under the same station address, Bus Master on again.

Teardown is stop (the I219 ring flush on the driver's own rings, then new
master requests blocked and waited out, both units stopped, DMA in flight
drained), Bus Master off (the masked Command write, which names the bits it
changes; the plain one only ORs bits on), reset, then `MkDeviceRelease`,
which is the real guard: it clears Bus Master with a read-back before
freeing every grant. A
setup that fails on a DMA grant, before any ring was programmed, stops an
older part the same way without the flush, and stops an I219's units
without the reset: a reset over descriptors it may still hold is the unit
hang the flush exists for, and the next attempt resets it with Bus Master
off as above.

## Failure model

No device: one line and `EXIT_ABSENT`. A failing step gives the device back
(once DMA grants are taken, stopped first as above), logs `e1000e: <step>`,
and the attempt is retried on the shared bounded schedule; running out is
`EXIT_GAVE_UP`. A hung transmitter is logged (`tx hang, reset: done` or the
step that failed), and a failed reset is tried again a whole watchdog bound
later. Link down and RX empty are answers, not errors.

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

- Signed by the owner and added to the image's features, with net.core,
  net.l2, the setup wizard and capsule_attack re-signed for the NIC table.
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
