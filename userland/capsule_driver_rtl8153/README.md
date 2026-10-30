# capsule_driver_rtl8153

Status: wired, in Standard (`microkernel-full-gui`), not yet signed. The kernel mirror
(`src/hardware/rtl8153_capsule`) embeds it under `nonos-capsule-driver-rtl8153` and spawns it where
a USB host is; net.core and net.l2 reach it through `src/services/registry/nics.rs`, and it reaches
driver.xhci0 as the other USB network capsules do. It has no owner certificate yet, so no image
carries it until the owner's seal writes one. Host-proven (`rtl8153_proofs`), not booted on
hardware: QEMU has no model of the chip.

## Role

`capsule_driver_rtl8153` drives Realtek RTL8153 and RTL8153B USB 3.0 gigabit Ethernet adapters in
their vendor configuration: the common laptop dongle, and the wired network for a machine whose
Wi-Fi NONOS cannot drive. It finds the adapter on driver.xhci0's ports, brings the chip up with
vendor control transfers, and serves Ethernet frames to net.core over NNET. The search, the xHCI
client and the NNET service are `nonos_usbnet`'s; this capsule holds the chip's register path, its
bring-up and its framing (`src/rtl`). How it binds, frames and follows the link, and what it logs:
[BINDING.md](BINDING.md).

```text
RTL8153 / RTL8153B USB adapter (vendor configuration)
  --> driver.xhci0 -- descriptors, vendor control, bulk OUT, polled bulk IN
  --> driver.rtl8153_0 -- one descriptor-framed packet per transfer --> net.core
```

The chip's behaviour is read from Linux v6.12 `drivers/net/usb/r8152.c`, for behaviour only: the
code here is written under its own names, with the bring-up as tables of register steps. Realtek
publishes no programming reference for the vendor mode.

## Microkernel contract

`CAPSULE_REQUIRED_CAPS = 0x38`: IPC (0x08), Memory (0x10) and Crypto (0x20), as cdc_ecm. No
CoreExec, InputSource, Driver, DeviceEnum, Mmio, Irq, Dma or Pio: the chip's registers are reached
through driver.xhci0's control transfers. Debug (0x100) is optional, granted only to a build that
compiles `capsule-serial-debug`. Its own pid is read back with `MkServiceLookup`, so no CoreExec. It receives requests with `MkIpcRecvFrom`, answers with `MkIpcReply`, and calls driver.xhci0 with `MkIpcCall`.

## Interface contract

NNET on `driver.rtl8153_0` (port 4258, reply port 4259, reply inbox `endpoint.4294967364`), as
every NIC driver speaks it: [PROTOCOL.md](PROTOCOL.md).

## Authority

Allowed: the chip's registers through vendor requests 05h, its descriptor framing, frame counters.
Forbidden: xHCI ownership, USB scheduling, DMA buffers, routing, sockets, IP policy. The kernel holds
`driver.rtl8153_0` to net.core and net.l2 through the NIC table, so no app reaches it, refuses it in
Air-Gapped, Safe and Recovery, and holds driver.xhci0 to its USB clients, this one among them.

## Privacy and persistence

The station address is drawn from `CryptoRandom` once a run, before the first bind, and written
into the chip's address register; the chip's own address, in its efuse, is never read. Without
entropy no device is bound. No log line carries a station address or a serial number; the
adapter's vendor:product appears only on a failed bind. Frames pass through and are not kept past
the request that carries them.

## Runtime lifecycle

As the other USB network capsules: started where a USB host is, it looks at every host's free ports
once a second, binds inside the 15 s bind budget, gives a device back after eight failures in a row
and looks again (hot-plug), and hands a rebound device the same address. A process exit takes
nothing with it but its own memory.

## Failure model

Every bind step that fails is named with its errno, and the device's slot is given back. Every wait
on the chip is a bounded number of looks (`src/rtl/bounds.rs`). A chip other than an RTL8153 or
RTL8153B is refused by name before the device is configured. Every receive length the chip gives
is bounded against the bytes that came; a stalled pipe is reset and its halt cleared.

## Current implemented surface

Binding to the 16 RTL8153-family ids in Linux v6.12's r8152 table, in the adapter's vendor configuration; the chip's version read first, and an RTL8152, RTL8153C, RTL8156 or RTL8156B refused by name before configuring; the bring-up as tables of register steps, every wait bounded (a whole bind under 9.5 s); the drawn station address written into the chip; receive walking Realtek's 24-byte descriptors with every length checked against the bytes received; send with the 8-byte descriptor; the link from `PLA_PHYSTATUS`. Not in the Standard profile: held until Decision 10 is ruled.

## Wire format

NNET requests carry the 20-byte header `magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32`; replies repeat it and add a 4-byte signed status, then the operation's data, as `nonos_usbnet` defines them for every USB network capsule.

## State ownership

`driver.rtl8153_0` owns the bound device's bulk pipes and interrupt endpoint, the run's drawn station address, and one receive and one send buffer. driver.xhci0 owns slots, endpoint contexts, rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core. Do not add hardware authority here. Bound every length the device sends before using it. Never read or log the chip's factory address or a serial. Log one line per bring-up step that can refuse, naming the step.

## Release target

`USB device -> driver.xhci0 -> driver.rtl8153_0 -> net.core`: DHCP and a web page over a real RTL8153 or RTL8153B adapter, once Decision 10 is ruled.

## Release evidence

Build and host-proof evidence only (rtl8153_proofs, 55 tests on a modelled chip, a mutant for every bound; usbnet_proofs for the shared core). QEMU has no RTL8153 and no real adapter has been plugged in.

## Release checklist

The capsule builds with zero warnings and is clippy clean; rtl8153_proofs passes; the maintainers rule on Decision 10 and, if it ships, signs it with net.core, net.l2, the setup wizard and capsule_attack re-signed for the NIC table; on hardware, `[usbnet rtl8153] port N up, drawn station address programmed` and `[NET-CORE] bind: interface up on driver.rtl8153_0`, then DHCP and a page (`docs/hardware/real-hardware-checklist.md`).

## Explicit non-goals today

Receive aggregation, checksum and segmentation offload, VLAN stripping, EEE, ALDPS and the chip's
other power saving, the PHY tuning and firmware patches Linux applies, jumbo frames, the RTL8152,
RTL8153C and RTL8156 families, more than one adapter per capsule.

## Verification

`cd userland/rtl8153_proofs && cargo test` (55 pass at the time of writing, with a mutant for
every bound); the capsule's release build and `cargo clippy` for x86_64-nonos-user, no warnings.
Not verified under QEMU or on hardware: [real-hardware checklist](../../docs/hardware/real-hardware-checklist.md).
