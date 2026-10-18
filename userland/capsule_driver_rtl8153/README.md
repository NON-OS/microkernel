# capsule_driver_rtl8153

Status: not in the 0.9.2 image. The capsule builds and its host proofs pass, but it has no owner certificate and the kernel never spawns it: driver.xhci0 has no bulk IN poll or control OUT data stage yet.

## Role

`capsule_driver_rtl8153` drives the Realtek RTL8153 and RTL8153B USB 3.0 Gigabit Ethernet chips in their vendor mode, as Linux `r8152` does. They are the chip of most USB-C Gigabit dongles and of dock Ethernet. It finds one on driver.xhci0's ports, brings it up, and serves Ethernet frames to net.core over NNET. The search, the xHCI client and the NNET service are `nonos_usbnet`'s; this capsule holds only the chip's register layer, its bring-up and its framing (`src/r8153`). How it binds, frames and follows the link, and what it logs: [BINDING.md](BINDING.md).

```text
RTL8153 adapter or dock
  --> driver.xhci0 -- descriptors, control, bulk OUT, polled bulk IN
  --> driver.rtl8153_0 -- tx_desc / rx_desc framing --> net.core
```

## Microkernel contract

The manifest's `CAPSULE_REQUIRED_CAPS = 0x38` grants IPC (0x08), Memory (0x10) and Crypto (0x20), and no InputSource, Driver, DeviceEnum, Mmio, Irq, Dma or Pio: it is a class capsule above xHCI. Debug (0x100) is its `CAPSULE_OPTIONAL_CAPS`, granted only to a build that compiles `capsule-serial-debug`. The capsule receives requests with `MkIpcRecvFrom` and answers with `MkIpcReply`, finds driver.xhci0 with `MkServiceLookup` and calls it with `MkIpcCall` (each call bounded at 12 s), draws its station address with `CryptoRandom`, which needs Crypto, writes its `[usbnet rtl8153]` lines with `MkDebug`, which needs Debug, and waits between bring-up steps with libc's `mk_idle_ms`. It makes no enumeration, MMIO, IRQ, DMA or PIO broker syscall: driver.xhci0 owns the controller.

## Interface contract

NNET, the protocol net.core speaks with every NIC driver, on `driver.rtl8153_0` (port 4258, reply port 4259): health (1), link (2), the station address (3), a frame out (4), a frame in (5), and the counters at `OP_STATS`, 0x0100, clear of net.core's receive batch at 6, which is answered `E_INVAL` as any operation it does not know. What each takes and answers: [PROTOCOL.md](PROTOCOL.md).

## Authority

Allowed: descriptor classification, RTL8153 binding and framing, frame counters. Forbidden: xHCI ownership, USB scheduling, DMA buffers, routing, sockets, IP policy. The kernel's held-endpoint table (`src/services/registry/held_table.rs`) has no entry for `driver.rtl8153_0` yet, as nothing spawns it; the PCI NICs' entries hold them to net.core and net.l2, and this one needs the same before it ships.

## Privacy and persistence

The station address is drawn from `CryptoRandom` once a run, before the first bind, and handed to every device bound in that run: locally administered unicast, never the address in PLA_BACKUP, which is never read. Without entropy no device is bound. No log line carries a station address, and the device's vendor:product appears only on a failed bind. Frames pass through and are not kept past the request that carries them; the capsule stores no product strings, serial numbers or frame contents. It holds the bound device's bulk pipes, the run's drawn station address, its chip version, the cached link state, and the frames of the last transfer received in process memory, dropped when the process ends.

## Runtime lifecycle

Once started it draws the run's station address, then looks for driver.xhci0. Until a device is bound the link reads down and frames answer `E_AGAIN`; every second the free root ports are looked at and each new device is offered to this driver, inside a bind budget of 15 s. A device whose transfers or link reads fail eight times in a row with no frame moved is given back and looked for again, and a rebind hands it the same address, the one net.core already uses. A port that fails three binds is left until it reconnects; every port decided against is tried again after a minute, then twice as long after each round that binds nothing, up to half an hour.

## Failure model

Every bind step that fails is a named failure with its errno on the log, and the device's slot is given back; once a bind's budget is spent no further call to driver.xhci0 starts. Malformed or hostile transfers from the device are dropped whole, never trusted past the bytes that came, and a reply from driver.xhci0 is read only as far as it came; a stalled pipe is reset on the controller and its halt cleared on the device.

## Current implemented surface

RTL8153 binding as the Linux driver binds it ([BINDING.md](BINDING.md)), and the framing: one frame per transfer out behind its 8-byte tx_desc; every frame of a transfer in behind its 24-byte rx_desc, checked against the transfer, CRC stripped, handed up one per request.

## Wire format

NNET requests carry the 20-byte header `magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32` (`NNET` magic, version 1, op, flags, request id, payload length); replies repeat it and add a 4-byte signed status, then the operation's data ([PROTOCOL.md](PROTOCOL.md)).

## State ownership

`driver.rtl8153_0` owns the bound device's bulk pipes, the run's drawn station address, its chip version, the cached link state, and the frames of the last transfer received. driver.xhci0 owns slots, endpoint contexts, rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core. Do not add hardware authority here. Bound every length the device sends before using it. Log one line per step a bind can stop at, and never a station address.

## Release target

`USB device -> driver.xhci0 -> driver.rtl8153_0 -> net.core`: DHCP and a web page over a real RTL8153 device.

## Release evidence

Build and host-proof evidence only so far (rtl8153_proofs, 31 tests; usbnet_proofs, 38 tests, for the shared core). Runtime evidence needs driver.xhci0's polled bulk IN and control OUT data stage (`docs/drivers/ethernet/usb-net.md`), the kernel spawn, and a real device: none is on main yet.

## Release checklist

The capsule builds with zero warnings and is clippy clean; rtl8153_proofs passes; driver.xhci0 serves OP_BULK_IN_POLL and control OUT data stages; the kernel spawns the capsule and holds its endpoint; on hardware, `[usbnet rtl8153] port N up, drawn station address programmed` and `[NET-CORE] bind: interface up on driver.rtl8153_0`, then DHCP and a page.

## Explicit non-goals today

RTL8152, RTL8156 and RTL8153C, firmware patches, power management (U1/U2, LPM, UPS, suspend), EEE, checksum offload, TSO, TX aggregation, MAC pass-through for docks.

## Verification

`cd userland/rtl8153_proofs && cargo test` (31 pass at the time of writing); the capsule's `cargo build --release` and `cargo clippy` for x86_64-nonos-user. Not verified under QEMU or on hardware.
