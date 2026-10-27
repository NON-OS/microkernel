# capsule_driver_ax88179

Status: wired, in the Standard image (`microkernel-full-gui`), not yet signed. The kernel mirror
(`src/hardware/ax88179_capsule`) embeds it under `nonos-capsule-driver-ax88179` and spawns it where a
certificate yet: `scripts/baselines/unsealed.txt` lists it until the release seal writes its trust
set. Host-proven only; not booted on hardware.

## Role

`capsule_driver_ax88179` drives the ASIX AX88179 (USB 3.0) and AX88178A (USB 2.0) gigabit Ethernet chips and the adapters sold under other names around them, one of the two most common USB Ethernet dongles: the wired way online for a laptop whose Wi-Fi NONOS cannot drive. It finds the chip on the USB hosts' ports, brings it up through its vendor register access, and serves Ethernet frames to net.core over NNET, the protocol every NONOS network driver speaks. The search, the USB host client and the NNET service are `nonos_usbnet`'s; this capsule holds the chip's binding, bring-up, framing and link (`src/ax`). ASIX publishes no programming reference: what the chip is sent is as Linux v6.12's `ax88179_178a.c` sends it, read for behaviour only, written here. How it binds, frames and follows the link, and what it logs: [BINDING.md](BINDING.md).

```text
AX88179 / AX88178A adapter
  --> driver.xhci0 (or another USB host) -- descriptors, vendor requests, bulk OUT, polled bulk IN, status IN
  --> driver.ax88179_0 -- 8-byte TX header; RX trailer, table and frames --> net.core
```

## Microkernel contract

The manifest's `CAPSULE_REQUIRED_CAPS = 0x38` grants IPC (0x08), Memory (0x10) and Crypto (0x20),
and no CoreExec, InputSource, Driver, DeviceEnum, Mmio, Irq, Dma or Pio: the chip is reached only
through the USB host's transfers. Debug (0x100) is its `CAPSULE_OPTIONAL_CAPS`, granted only to a
build that compiles `capsule-serial-debug`. The capsule receives requests with `MkIpcRecvFrom` and
answers with `MkIpcReply`, finds every USB host and its own pid with `MkServiceLookup` and calls it
with `MkIpcCall` (each call bounded at 12 s), waits the bring-up's fixed pauses with a futex wait on a private word (`mk_idle_ms`, as rndis waits), draws
its station address with `CryptoRandom`, which needs Crypto, and writes its `[usbnet ax88179]` lines
with `MkDebug`, which needs Debug.

## Interface contract

NNET on `driver.ax88179_0` (port 4256, reply port 4257): health (1), link (2), the station address (3), a frame out (4), a frame in (5), and the counters at `OP_STATS`, 7. What each takes and answers: [PROTOCOL.md](PROTOCOL.md).

## Authority

Allowed: descriptor classification, the chip's vendor register access over endpoint 0, its framing,
frame counters. Forbidden: USB host ownership, DMA buffers, routing, sockets, IP policy. The kernel
holds `driver.ax88179_0` to the wired stack (net.core, net.l2) through the NIC table, and the USB
hosts to their class drivers (`src/services/registry/held_lists.rs`), this one among them.

## Privacy and persistence

The station address is drawn from `CryptoRandom` once a run, before the first bind, and written into the chip's station register; the factory address is never read. Without entropy no device is bound. No log line carries a station address or a serial number, and the device's vendor:product appears only on a failed bind. Frames pass through and are not kept past the request that carries them.

## Runtime lifecycle

As every `nonos_usbnet` driver: until a chip is bound the link reads down and frames answer `E_AGAIN`; every second the free root ports are looked at, each new device offered inside a 15 s bind budget. A chip unplugged, or whose transfers and PHY reads fail eight times in a row with no frame moved, is given back and looked for again; a rebind writes the same drawn address. A port that fails three binds is left until it reconnects.

## Failure model

Every bind step that fails is named on the log with its errno, and the device's slot is given back. Every wait is a fixed pause or a bounded count of rounds. Every length and offset the chip sends is bounded against the bytes received before it is used: a lying trailer or table drops the transfer whole, never read past it. A stalled bulk pipe is reset on the host and its halt cleared on the device.

## Release evidence

Host proofs only: `userland/ax88179_proofs` mounts `src/ax` by path and holds the bring-up to a model of its vendor requests, the receive framing to a model under 100 000 random and hostile transfers, the send header, and the link; each bound has a mutant that fails it. Not booted under QEMU (it has no AX88179) or on hardware: `docs/hardware/real-hardware-checklist.md` has the test plan.

## Current implemented surface


## Wire format

NNET requests carry the 20-byte header `magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32`; replies repeat it and add a 4-byte signed status, then the operation's data, as `nonos_usbnet` defines them for every USB network capsule.

## State ownership

`driver.ax88179_0` owns the bound device's bulk pipes and interrupt endpoint, the run's drawn station address, and one receive and one send buffer. driver.xhci0 owns slots, endpoint contexts, rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core. Do not add hardware authority here. Bound every length the device sends before using it. Never read or log the chip's factory address or a serial. Log one line per bring-up step that can refuse, naming the step.

## Release target

`USB device -> driver.xhci0 -> driver.ax88179_0 -> net.core`: DHCP and a web page over a real AX88179 or AX88178A dongle, once Decision 10 is ruled.

## Release checklist


## Explicit non-goals today

Checksum and segmentation offload, jumbo frames, a hashed multicast list, wake-on-LAN, EEE, more than one chip per capsule.

## Verification

`cd userland/ax88179_proofs && cargo test`; the capsule's build line from `Capsule.mk` with zero warnings, and `cargo clippy`.
