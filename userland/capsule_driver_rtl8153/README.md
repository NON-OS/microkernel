# capsule_driver_rtl8153

## Role

`capsule_driver_rtl8153` drives the Realtek RTL8153 and RTL8153B USB 3.0
Gigabit Ethernet chips in their vendor mode, as Linux `r8152` does. They
are the chip of most USB-C Gigabit dongles and of dock Ethernet. It finds
one on driver.xhci0's ports, brings it up, and serves Ethernet frames to
net.core over NNET.

```text
RTL8153 adapter or dock
        |
        v
driver.xhci0 -- descriptors, control, bulk OUT, polled bulk IN
        |
        v
driver.rtl8153_0 -- tx_desc / rx_desc framing --> net.core
```

The search, the xHCI client and the NNET service are `nonos_usbnet`'s;
this capsule holds the chip's register layer, its bring-up and its
framing (`src/r8153`).

## Binding

1. The adapter by vendor:product, from r8152's `rtl8152_table` (`ids.rs`).
2. The vendor configuration: the one whose first interface is class 0xFF,
   with bulk IN on endpoint 1 and bulk OUT on endpoint 2.
3. The chip version from PLA_TCR0; RTL_VER_03 to 06 (RTL8153) and 08, 09
   (RTL8153B) are taken, every other version is refused by name.
4. The bulk pipes go to driver.xhci0, then SET_CONFIGURATION.
5. The bring-up in Linux's order (`up/bring_up.rs`), the station address
   from PLA_BACKUP written to PLA_IDR.
6. When the link comes up, TX and RX are enabled (`enable/`).

## Framing

TX: one frame per bulk transfer, behind an 8-byte tx_desc (TX_FS, TX_LS,
length). No padding, as r8152. RX: RX aggregation is turned off so each
frame comes in its own transfer under BULK_MAX; every transfer is still
walked as r8152's rx_bottom walks it (24-byte rx_desc, 8-byte steps, CRC
stripped), and its frames are handed up one per receive.

The link is read from PLA_PHYSTATUS at most once a second and kept.

## Log

`log usbnet` shows `[usbnet rtl8153]` lines: each port tried, a refused
chip or failed step by name with its errno, `chip RTL8153 (RTL_VER_0x)`
and `port N up, mac ...` when bound, `link up, 1000 Mb/s full duplex` and
`link down`.

## Contract

Caps `0x200018` (IPC, Memory, Debug). Service `driver.rtl8153_0` on port
4258, reply port 4259. Proofs: `userland/rtl8153_proofs`. Hardware notes:
`docs/hardware/rtl8153.md`.

## Microkernel contract

The manifest grants `IPC`, `Memory` and `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0x200018
```

The capsule receives requests with `MkIpcRecvFrom` and answers with
`MkIpcReply`, finds driver.xhci0 with `MkServiceLookup` and calls it with
`MkIpcCall` (bounded by a timeout), writes its `[usbnet rtl8153]` lines with
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
allowed:   descriptor classification, RTL8153 binding and framing, frame counters
forbidden: xHCI ownership, USB scheduling, DMA buffers, routing, sockets, IP policy
```

The kernel holds `driver.rtl8153_0` to the wired stack (net.core, net.l2), so no
app reaches it.

## Privacy and persistence

Frames pass through and are not kept past the request that carries them. The
capsule stores no product strings, serial numbers or frame contents; it holds
the bound device's bulk pipes, its MAC, its chip version, the cached link state, and the frames of the last transfer received in process memory, dropped when the process ends.

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

RTL8153 binding as the Linux driver binds it (README sections above, and
docs/hardware/), and the framing: one frame per transfer out behind its 8-byte tx_desc; every frame of a transfer in behind its 24-byte rx_desc, checked against the transfer, CRC stripped, handed up one per request.

## Wire format

NNET requests carry the 20-byte header (`NNET` magic, version 1, op, flags,
request id, payload length); replies repeat it and add a 4-byte signed status:

```text
magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32
```

## State ownership

`driver.rtl8153_0` owns the bound device's bulk pipes, its MAC, its chip version, the cached link state, and the frames of the last transfer received. driver.xhci0 owns slots, endpoint contexts,
rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

- Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core.
- Do not add hardware authority here.
- Bound every length the device sends before using it.
- Log one line per step a bind can stop at.

## Release target

```text
USB device -> driver.xhci0 -> driver.rtl8153_0 -> net.core
```

DHCP and a web page over a real RTL8153 device.

## Release evidence

Build and host-proof evidence only so far (rtl8153_proofs, 27 tests).
Runtime evidence needs driver.xhci0's polled bulk IN and control OUT data
stage (docs/hardware/usb-net.md), the kernel spawn, and a real device: none
is on main yet.

## Release checklist

- Capsule builds with zero warnings; clippy clean.
- rtl8153_proofs passes.
- driver.xhci0 serves OP_BULK_IN_POLL and control OUT data stages.
- The kernel spawns the capsule and holds its endpoint.
- On hardware: `[usbnet rtl8153] port N up, mac ...` and
  `[NET-CORE] bind: interface up on driver.rtl8153_0`, then DHCP and a page.

## Explicit non-goals today

RTL8152, RTL8156 and RTL8153C, firmware patches, power management (U1/U2, LPM, UPS, suspend), EEE, checksum offload, TSO, TX aggregation, MAC pass-through for docks.

## Verification

`cd userland/rtl8153_proofs && cargo test` (27 pass at the time of
writing); the capsule's `cargo build --release` and `cargo clippy` for
x86_64-nonos-user. Not verified under QEMU or on hardware.
