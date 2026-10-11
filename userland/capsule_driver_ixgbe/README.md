# capsule_driver_ixgbe

Status: not in the image. The capsule builds and its host proofs pass, but
it has no owner certificate, no kernel embed and no spawn yet.

## Role

`capsule_driver_ixgbe` is the Intel 82599 (X520), X540 and X550 10GbE
Ethernet driver capsule (Linux `ixgbe`). It owns the PCI function, BAR0, one
advanced RX queue, one advanced TX queue and raw Ethernet frame movement,
and stops at the frame boundary. QEMU has no ixgbe device: this driver runs
on hardware only.

```text
net.core / net.l2
    |
    | raw Ethernet IPC (NNET, as driver.e1000_0)
    v
driver.ixgbe_0 -- broker MMIO + DMA rings --> 82599 / X540 / X550
    |
    `-- polled: no interrupt is bound
```

## Microkernel contract

- `MkDeviceList` finds the first Intel Ethernet function whose ID is in the
  table: the 82599, X540 and X550 entries of Linux `ixgbe_pci_tbl`.
- `MkDeviceClaim` claims it; `MkPciConfigWrite` turns on Memory Space and
  Bus Master Enable.
- `MkMmioMap` maps BAR0; `MkDmaMap` takes the RX ring, RX buffers, TX ring
  and TX buffers.
- `MkIpcRecv` and `MkIpcSend` serve `service:4276:driver.ixgbe_0`.
- `CryptoRandom` draws the station address each boot.

No interrupt is bound and every cause is masked (EIMC and EIMC_EX).

## Interface contract

Same operations as `driver.e1000_0`: `OP_HEALTHCHECK`, `OP_LINK_STATUS`
(1 byte), `OP_MAC_ADDRESS` (6 bytes), `OP_TX_PACKET`, `OP_RX_PACKET`
(length and frame), `OP_STATS` (48-byte snapshot, with RXCTRL and DMATXCTL
where e1000 has RCTL and TCTL).

## Authority

`CAPSULE_REQUIRED_CAPS = 0xB8038`: IPC, Memory, Crypto, Driver,
DeviceEnum, Mmio, Dma. `CAPSULE_OPTIONAL_CAPS = 0x100` (Debug) is granted
only by a `capsule-serial-debug` build; without it the `ixgbe:` lines are
dropped. No Irq, Network, filesystem or admin authority.

## Privacy and persistence

Frames live in DMA buffers and IPC payloads only while they are handled.
The factory address is never used: a locally administered address is drawn
each boot, before either queue is enabled, and bring-up fails closed without
entropy. Nothing is stored.

## Runtime lifecycle

Discover, claim, enable bus mastering, map, take DMA grants, then bring the
part up in Linux's order:

1. stop the adapter and the PCIe master, then the double MAC reset and the
   EEPROM auto-read;
2. on the 82599, put back the AUTOC link mode found before the reset;
3. DRV_LOAD and no-snoop disable;
4. the address and filters;
5. HLREG0, RDRXCTL, FCTRL;
6. DMATXCTL.TE and the TX queue;
7. the RX queue, then RXCTRL.RXEN, inside the 82599's security-block hold.

Then serve IPC until the process ends.

## Failure model

No device: one line and `EXIT_ABSENT`. A failing step stands the part down,
gives back every grant, logs `ixgbe: <step>`, and the attempt is retried on
the shared bounded schedule; running out is `EXIT_GAVE_UP`. Link down and
RX empty are answers, not errors.

## Current implemented surface

Device table for 17 IDs over three MACs, polled RX/TX on advanced one-buffer
descriptors with 2048-byte buffers, CRC stripped, short frames padded, link
lines from LINKS (100 Mb/s to 10 Gb/s, and 2.5 and 5 Gb/s on the X550).

## Wire format

As `capsule_driver_e1000`: 20-byte NNET header, 4-byte status, per-op data.

## State ownership

The capsule owns the rings, buffers, register state and station address;
`net.core` and `net.l2` own protocol interpretation; the kernel owns grants.

## Operating rules

Frames only; no ARP, IP or socket logic here. Every wait is on the uptime
clock. Every step that can fail logs one `ixgbe:` line.

## Release target

A signed, spawned raw-frame NIC service for X520, X540 and X550 cards in
servers and workstations.

## Release evidence

A real boot on one X520 (SFP+ with a DA cable), one X540 or X550 machine,
each with `log ixgbe` showing the `up` line and a `link up` line, and DHCP
completing through `net.core`.

## Release checklist

- Kernel inventory family, mirror, spawn and NIC table entry.
- Signed by the owner and added to the image's features.
- One machine of each MAC photographed with those lines.

## Explicit non-goals today

No interrupts, MSI-X, multiple queues, DCB, VMDq or SR-IOV, jumbo frames,
flow control, offloads, RSS, timestamping, EEPROM or PHY access. No SFP+
module identification or setup: an 82599 SFP+ port runs in the mode firmware
left in AUTOC, which a DA cable usually needs nothing more for and an optical
module may. No 82598 and no X550EM (firmware- or external-PHY-managed
links). Only the first ixgbe function in the device list is taken.

## Verification

- Build: `cargo build --release --target ../x86_64-nonos-user.json
  -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem`
- Proofs: `cd userland/ixgbe_proofs && cargo test`
- Not verified on any hardware.
