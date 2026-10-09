# capsule_driver_ax88179

Status: not in the 0.9.2 image. The capsule builds and its host proofs
pass, but it has no owner certificate and the kernel never spawns it:
driver.xhci0 has no bulk IN poll or control OUT data stage yet.

## Role

`capsule_driver_ax88179` drives the ASIX AX88179 (USB 3.0) and AX88178A
(USB 2.0) Gigabit Ethernet chips, the chip in most USB-C and USB-A Gigabit
dongles. It finds one on driver.xhci0's ports, brings it up as Linux
`ax88179_178a` does, and serves Ethernet frames to net.core over NNET.

```text
AX88179 / AX88178A dongle
        |
        v
driver.xhci0 -- descriptors, vendor control, bulk OUT, polled bulk IN
        |
        v
driver.ax88179_0 -- 8-byte TX header, aggregated RX --> net.core
```

The search, the xHCI client and the NNET service are `nonos_usbnet`'s; this
capsule holds only the chip (`src/ax`).

## Binding

1. The vendor:product is in Linux's products[] (13 entries, `products.rs`).
2. The configuration Linux would choose (`usb_choose_configuration`) holds
   an interface of class ff/ff/00 with a bulk IN and a bulk OUT pipe.
3. Pipes to driver.xhci0, SET_CONFIGURATION, SET_INTERFACE (answer ignored).
4. `ax88179_reset` in order: PHY power and clocks (200 and 100 ms waits),
   auto detach when the EEPROM asks, the MAC from AX_NODE_ID, bulk IN
   aggregation, pause levels, checksum offload off, AX_RX_CTL, monitor
   mode, default medium, EEE off, auto-negotiation restarted.

A chip whose AX_NODE_ID is not a station address is refused: Linux draws a
random one, and this capsule holds no Crypto capability to draw it.

## Framing

TX: an 8-byte header (length, zero MSS word) before each frame; a transfer
that fills its last packet gets one padding byte and the header's padding
flags, as Linux sends it.

RX: one bulk IN of at most 4096 bytes carries several frames, found from
the header at its tail. Every count, offset and length is checked; a
malformed transfer is dropped whole. Frames are queued and handed up one
per receive.

## Link

No interrupt endpoint is configured, so the PHY is read on the clock, at
most once a second, after a request is answered (`Nic::tick`). `link_up`
answers from that reading. On a new link the medium mode and bulk IN row
are set as `ax88179_link_reset` sets them.

## Contract

Caps `0x200018` (IPC, Memory, Debug). Service `driver.ax88179_0` on port
4256, reply port 4257. Proofs: `userland/ax88179_proofs`. Page:
`docs/hardware/ax88179.md`.

## Microkernel contract

The manifest grants `IPC`, `Memory` and `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0x200018
```

The capsule receives requests with `MkIpcRecvFrom` and answers with
`MkIpcReply`, finds driver.xhci0 with `MkServiceLookup` and calls it with
`MkIpcCall` (bounded by a timeout), writes its `[usbnet ax88179]` lines with
`MkDebug`, and waits on the
clock between bring-up steps with libc's `mk_idle_ms`. It makes no enumeration, MMIO, IRQ, DMA
or PIO broker syscall: driver.xhci0 owns the controller.

## Interface contract

NNET, the protocol net.core speaks with every NIC driver:

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` (1) | none | status |
| `OP_LINK_STATUS` (2) | none | one byte: 1 up, 0 down or no device bound |
| `OP_MAC_ADDRESS` (3) | none | six bytes, or `E_AGAIN` with no device |
| `OP_TX_PACKET` (4) | one Ethernet frame, 14 to 1514 bytes | status |
| `OP_RX_PACKET` (5) | none | `len_le32` and the frame, or `E_AGAIN` |
| `OP_STATS` (6) | none | twelve 32-bit counters |

## Authority

```text
allowed:   descriptor classification, AX88179 / AX88178A binding and framing, frame counters
forbidden: xHCI ownership, USB scheduling, DMA buffers, routing, sockets, IP policy
```

The kernel holds `driver.ax88179_0` to the wired stack (net.core, net.l2), so no
app reaches it.

## Privacy and persistence

Frames pass through and are not kept past the request that carries them. The
capsule stores no product strings, serial numbers or frame contents; it holds
the bound device's bulk pipes, its MAC, the cached link state and speed, and the frames of the last transfer received in process memory, dropped when the process ends.

## Runtime lifecycle

Started wherever an xHCI controller is. Until a device is bound the link
reads down and frames answer `E_AGAIN`; every second the free root ports are
looked at and each new device is offered to this driver. A device that fails
eight transfers in a row is given back and looked for again; a port that
fails three binds is left until it reconnects.

## Failure model

Every bind step that fails is a named failure with its errno on the log, and
the device's slot is given back. Malformed or hostile transfers from the
device are dropped whole, never trusted past the bytes that came; a stalled
pipe is reset on the controller and its halt cleared on the device.

## Current implemented surface

AX88179 / AX88178A binding as the Linux driver binds it (README sections above, and
docs/hardware/), and the framing: one frame per transfer out behind an 8-byte header; the packets of a transfer in, found from the tail header and checked against the transfer, handed up one per request.

## Wire format

NNET requests carry the 20-byte header (`NNET` magic, version 1, op, flags,
request id, payload length); replies repeat it and add a 4-byte signed status:

```text
magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32
```

## State ownership

`driver.ax88179_0` owns the bound device's bulk pipes, its MAC, the cached link state and speed, and the frames of the last transfer received. driver.xhci0 owns slots, endpoint contexts,
rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

- Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core.
- Do not add hardware authority here.
- Bound every length the device sends before using it.
- Log one line per step a bind can stop at.

## Release target

```text
USB device -> driver.xhci0 -> driver.ax88179_0 -> net.core
```

DHCP and a web page over a real AX88179 / AX88178A device.

## Release evidence

Build and host-proof evidence only so far (ax88179_proofs, 25 tests).
Runtime evidence needs driver.xhci0's polled bulk IN and control OUT data
stage (docs/hardware/usb-net.md), the kernel spawn, and a real device: none
is on main yet.

## Release checklist

- Capsule builds with zero warnings; clippy clean.
- ax88179_proofs passes.
- driver.xhci0 serves OP_BULK_IN_POLL and control OUT data stages.
- The kernel spawns the capsule and holds its endpoint.
- On hardware: `[usbnet ax88179] port N up, mac ...` and
  `[NET-CORE] bind: interface up on driver.ax88179_0`, then DHCP and a page.

## Explicit non-goals today

LED setup, EEE, jumbo frames, Wake-on-LAN, suspend and resume, a multicast hash filter, TSO, checksum offload, a random MAC.

## Verification

`cd userland/ax88179_proofs && cargo test` (25 pass at the time of
writing); the capsule's `cargo build --release` and `cargo clippy` for
x86_64-nonos-user. Not verified under QEMU or on hardware.
