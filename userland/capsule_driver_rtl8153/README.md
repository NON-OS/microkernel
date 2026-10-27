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
compiles `capsule-serial-debug`. Its own pid is read back with `MkServiceLookup`, so no CoreExec.

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

## Explicit non-goals today

Receive aggregation, checksum and segmentation offload, VLAN stripping, EEE, ALDPS and the chip's
other power saving, the PHY tuning and firmware patches Linux applies, jumbo frames, the RTL8152,
RTL8153C and RTL8156 families, more than one adapter per capsule.

## Verification

`cd userland/rtl8153_proofs && cargo test` (55 pass at the time of writing, with a mutant for
every bound); the capsule's release build and `cargo clippy` for x86_64-nonos-user, no warnings.
Not verified under QEMU or on hardware: [real-hardware checklist](../../docs/hardware/real-hardware-checklist.md).
