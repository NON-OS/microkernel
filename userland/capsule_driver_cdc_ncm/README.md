# capsule_driver_cdc_ncm

Status: wired, not yet signed. The kernel mirror (`src/hardware/cdc_ncm_capsule`) embeds it under
`nonos-capsule-driver-cdc-ncm` and spawns it where an xHCI controller is; net.core and net.l2 reach
it through `src/services/registry/nics.rs`, and it reaches driver.xhci0, which serves its polled
bulk IN and control OUT data stage. It has no owner certificate yet, so no image carries it.

## Role

`capsule_driver_cdc_ncm` is the USB CDC Network Control Model class capsule. It finds one NCM function on driver.xhci0's ports, binds it as NCM 1.0 describes (as recalled, unverified), and serves Ethernet frames to net.core over NNET, the protocol every NONOS network driver speaks. The search, the xHCI client and the NNET service are `nonos_usbnet`'s; this capsule holds only the NCM binding and NTB framing (`src/ncm`). How it binds, frames and follows the link, and what it logs: [BINDING.md](BINDING.md).

```text
phone tethering over NCM, or an NCM USB Ethernet adapter
  --> driver.xhci0 -- descriptors, control, bulk OUT, polled bulk IN
  --> driver.cdc_ncm0 -- NTB16 blocks <-> one frame per call --> net.core
```

## Microkernel contract

The manifest's `CAPSULE_REQUIRED_CAPS = 0x38` grants IPC (0x08), Memory (0x10) and Crypto (0x20),
and no CoreExec, InputSource, Driver, DeviceEnum, Mmio, Irq, Dma or Pio: it is a class capsule above
xHCI. Debug (0x100) is its `CAPSULE_OPTIONAL_CAPS`, granted only to a build that compiles
`capsule-serial-debug`. The capsule receives requests with `MkIpcRecvFrom` and answers with
`MkIpcReply`, finds every USB host and its own pid with `MkServiceLookup` and calls it with
`MkIpcCall` (each call bounded at 12 s), draws its station address with `CryptoRandom`, which needs
Crypto, writes its `[usbnet cdc-ncm]` lines with `MkDebug`, which needs Debug, and waits between
bring-up steps with libc's `mk_idle_ms`. It makes no enumeration, MMIO, IRQ, DMA or PIO broker
syscall: driver.xhci0 owns the controller. Its own pid, by which the loop drops what it sent itself,
is read back by its service name rather than through `MkGetPid`, so it needs no CoreExec.

## Interface contract

NNET, the protocol net.core speaks with every NIC driver, on `driver.cdc_ncm0` (port 4252, reply port 4253): health (1), link (2), the station address (3), a frame out (4), a frame in (5), and the counters at `OP_STATS`, 7, clear of net.core's receive batch at 6, which is answered `E_INVAL` as any operation it does not know. What each takes and answers: [PROTOCOL.md](PROTOCOL.md).

## Authority

Allowed: descriptor classification, CDC-NCM binding and framing, frame counters. Forbidden: xHCI
ownership, USB scheduling, DMA buffers, routing, sockets, IP policy. The kernel holds
`driver.cdc_ncm0` to the wired stack (net.core, net.l2) in `src/services/registry/held_table.rs`,
through the NIC table (`src/services/registry/nics.rs`) as it holds the PCI NICs, so no app reaches
it; it holds driver.xhci0 to its USB class drivers, this one among them.

## Privacy and persistence

The station address is drawn from `CryptoRandom` once a run, before the first bind, and handed to every device bound in that run: locally administered unicast, never the device's iMACAddress string, which is never read. Without entropy no device is bound. No log line carries a station address, and the device's vendor:product appears only on a failed bind. Frames pass through and are not kept past the request that carries them; the capsule stores no product strings, serial numbers or frame contents. It holds the bound device's bulk pipes, the run's drawn station address, the negotiated NTB limits, and the datagrams of the last NTB received in process memory, dropped when the process ends.

## Runtime lifecycle

Started wherever an xHCI controller is, it draws the run's station address, then looks for the USB hosts, fastest first. Until a device is bound the link reads down and frames answer `E_AGAIN`; every second the free root ports of every registered host are looked at and each new device is offered to this driver, inside a bind budget of 15 s. A device whose transfers fail eight times in a row with no frame moved is given back and looked for again, and a rebind hands it the same address, the one net.core already uses. A port that fails three binds is left until it reconnects; every port decided against is tried again after a minute, then twice as long after each round that binds nothing, up to half an hour.

## Failure model

Every bind step that fails is a named failure with its errno on the log, and the device's slot is given back; once a bind's budget is spent no further call to driver.xhci0 starts. Malformed or hostile transfers from the device are dropped whole, never trusted past the bytes that came, and a reply from driver.xhci0 is read only as far as it came; a stalled pipe is reset on the controller and its halt cleared on the device.

## Current implemented surface

CDC-NCM binding as NCM 1.0 describes it ([BINDING.md](BINDING.md)), and the framing: one datagram per 16-bit NTB out; every NDP and datagram of a 16- or 32-bit NTB in, checked against the block and handed up one per request; the link from NETWORK_CONNECTION.

## Wire format

NNET requests carry the 20-byte header `magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32` (`NNET` magic, version 1, op, flags, request id, payload length); replies repeat it and add a 4-byte signed status, then the operation's data ([PROTOCOL.md](PROTOCOL.md)).

## State ownership

`driver.cdc_ncm0` owns the bound device's bulk pipes, the run's drawn station address, the negotiated NTB limits, and the datagrams of the last NTB received. driver.xhci0 owns slots, endpoint contexts, rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core. Do not add hardware authority here. Bound every length the device sends before using it. Log one line per step a bind can stop at, and never a station address.

## Release target

`USB device -> driver.xhci0 -> driver.cdc_ncm0 -> net.core`: DHCP and a web page over a real CDC-NCM device.

## Release evidence

Build and host-proof evidence only so far (cdc_ncm_proofs, 46 tests; usbnet_proofs, 53 tests, for
the shared core). driver.xhci0 serves the polled bulk IN and control OUT data stage and the kernel
spawns the capsule (`docs/drivers/ethernet/usb-net.md`); runtime evidence still needs an owner
certificate and a real device.

## Release checklist

The capsule builds with zero warnings and is clippy clean; cdc_ncm_proofs passes; the owner signs
it, with net.core, net.l2, the setup wizard and capsule_attack re-signed for the NIC table; on
hardware, `[usbnet cdc-ncm] port N up, drawn station address programmed` (on a device without
SET_NET_ADDRESS, `drawn station address, device filter opened`) and `[NET-CORE] bind: interface up
on driver.cdc_ncm0`, then DHCP and a page.

## Explicit non-goals today

32-bit NTBs out (they are read in), more than one datagram per NTB out, blocks over 4096 bytes, MBIM.

## Verification

`cd userland/cdc_ncm_proofs && cargo test` (46 pass at the time of writing); the capsule's `cargo build --release` and `cargo clippy` for x86_64-nonos-user. Not verified under QEMU or on hardware.
