# capsule_driver_tg3

Status: wired (`src/hardware/tg3_capsule`, `microkernel-driver-tg3`, nics.rs)
but not signed, so no image carries it yet.

## Role
Broadcom 57765-class GbE (Linux `tg3`): BCM57761, 57762, 57765, 57766,
57781, 57782, 57785, 57786, 57791, 57795 (last two 10/100); not in QEMU.

```text
net.core / net.l2 --NNET IPC--> driver.tg3_0 --broker MMIO + DMA--> 57765
                                      `-- polled status block, no IRQ bound
```

## Microkernel contract
`MkDeviceList` finds the first Broadcom Ethernet function in the table;
`MkDeviceClaim`; `MkPciConfigWrite` for Command and for the words tg3's chip
reset writes (host control, PCI state, PCIe Device Status, Relaxed Ordering
and No Snoop cleared), each inside the broker's quirk_bits; `MkMmioMap` for
BAR0; `MkDmaMap` for three grants; `MkIpcRecv` on 4506; `CryptoRandom`.

## Interface contract
As `driver.e1000_0`: health, link (1 byte), MAC (6 bytes), TX, RX (length
and frame), and stats on op 7 (48 bytes). Op 6 is net.core's OP_RX_BATCH.

## Authority
`CAPSULE_REQUIRED_CAPS = 0xB8038`, each bit named in `Capsule.mk`; Debug
(0x100) optional, for the `tg3:` lines. No Irq, Network or Pio.

## Privacy and persistence
The factory address is never used or printed: one is drawn per bring-up.
Frames live in grants and IPC only while handled. Nothing is stored.

## Runtime lifecycle
Claim, map, identify (revision from the product-ID register; other classes
refused before any register write), grants, then tg3_reset_hw's order:
firmware told, PHY and chip reset, 57765 fixups, rings, buffer manager,
address, engines, receiver, copper PHY, filters. Then serve, with a 1 s
tick: fresh status block, ASF heartbeat, link look.

## Failure model
No part: `EXIT_ABSENT`. A failed step logs `tg3: <step>`, stands the part
down (engines stopped, chip reset), gives every grant back, and retries on
the shared schedule; then `EXIT_GAVE_UP`. Link down and no frame are answers.

## Current implemented surface
One standard ring (64 buffers posted of 512), its return ring, one send
ring (63 frames in flight), 2 KiB slots, autonegotiated copper with pause.

## Wire format
As e1000's: 20-byte NNET header, 4-byte status, per-op data.

## State ownership
Here: rings, buffers, mode words, address. Stack: protocols. Kernel: grants.

## Operating rules
Frames only; every wait bounded on the uptime clock.

## Release target
A signed raw-frame NIC service for 57765-class laptops and desktops.

## Release evidence
A boot on one 57765-class machine: `tg3: up`, `tg3: link up`, DHCP.

## Release checklist
Owner keys; stack capsules re-signed for the NIC table; a photographed boot.

## Explicit non-goals today
No interrupts, MSI-X, RSS, TSO, checksum offload, jumbo frames, EEE, WoL,
multicast or VLAN stripping (tags stay in the frame). No APE parts (5717
class, 5762, 5725, 5727), no fiber, no other tg3 family; no 57766 patch.

## Verification
Build as every capsule (`--target ../x86_64-nonos-user.json`); proofs in
`userland/tg3_proofs` (`cargo test`). Not verified on any hardware.
