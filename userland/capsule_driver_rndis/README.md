# capsule_driver_rndis

Status: not in the 0.9.2 image. The capsule builds and its host proofs
pass, but it has no owner certificate and the kernel never spawns it:
driver.xhci0 has no bulk IN poll or control OUT data stage yet.

## Role

`capsule_driver_rndis` is the USB RNDIS (Microsoft Remote NDIS) class
capsule. It finds one RNDIS function on driver.xhci0's ports, binds it as
Linux `rndis_host` does, and serves Ethernet frames to net.core over NNET,
the protocol every NONOS network driver speaks.

```text
Android phone tethering over RNDIS, QEMU usb-net
        |
        v
driver.xhci0 -- descriptors, control, bulk OUT, polled bulk IN
        |
        v
driver.rndis0 -- PACKET_MSG framing, one frame per receive --> net.core
```

The search, the xHCI client and the NNET service are `nonos_usbnet`'s; this
capsule holds only the RNDIS binding, control channel and framing
(`src/rndis`).

## Binding

1. The first configuration with a control interface of class 0x02/0x02/0xFF
   (an ACM descriptor with capabilities is a modem and is left), 0xE0/0x01/0x03
   or 0xEF/0x04/0x01, and its data interface: the Union's, else the call
   management descriptor's bDataInterface, else interface 1 for a control
   interface 0. QEMU's usb-net offers RNDIS first (bConfigurationValue 2).
2. The bulk pipes go to driver.xhci0, then SET_CONFIGURATION. No
   SET_INTERFACE unless the pipes sit on a nonzero alternate setting.
3. INITIALIZE (RNDIS 1.0, host MaxTransferSize 4096), then the completion's
   status, medium (802.3) and MaxTransferSize (a full frame must fit).
4. QUERY OID_802_3_PERMANENT_ADDRESS: six bytes, a unicast address, or the
   bind fails.
5. SET OID_GEN_CURRENT_PACKET_FILTER: directed, broadcast, all multicast.

Each command is SEND_ENCAPSULATED_COMMAND, then GET_ENCAPSULATED_RESPONSE
polled every 10 ms for up to a second, matching the type and RequestID and
passing over INDICATE_STATUS messages. A failure after INITIALIZE sends
HALT.

## Framing

One PACKET_MSG per frame: a 44-byte header, DataOffset 36. A transfer that
fills its last packet gets one padding byte, counted in MessageLength. A
bulk IN of 4096 bytes may hold several messages; each is checked against
the transfer and its frame against the message, and the frames go up one
per receive. A malformed message is dropped.

The link reads up once bound: media state comes on the interrupt endpoint,
which driver.xhci0 does not configure beside bulk pipes.

## Log

`log usbnet` shows `[usbnet rndis]` lines: each port tried and why it was
not taken, `port N up, mac ...` when bound, and `device stopped answering`
when it is given back.

## Contract

Caps `0x200018` (IPC, Memory, Debug). Service `driver.rndis0` on port 4254,
reply port 4255. Proofs: `userland/rndis_proofs`. Notes:
`docs/hardware/rndis.md`.

## Microkernel contract

The manifest grants `IPC`, `Memory` and `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0x200018
```

The capsule receives requests with `MkIpcRecvFrom` and answers with
`MkIpcReply`, finds driver.xhci0 with `MkServiceLookup` and calls it with
`MkIpcCall` (bounded by a timeout), writes its `[usbnet rndis]` lines with
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
allowed:   descriptor classification, RNDIS binding and framing, frame counters
forbidden: xHCI ownership, USB scheduling, DMA buffers, routing, sockets, IP policy
```

The kernel holds `driver.rndis0` to the wired stack (net.core, net.l2), so no
app reaches it.

## Privacy and persistence

Frames pass through and are not kept past the request that carries them. The
capsule stores no product strings, serial numbers or frame contents; it holds
the bound device's bulk pipes, its MAC, the RNDIS request id, and the frames of the last transfer received in process memory, dropped when the process ends.

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

RNDIS binding as the Linux driver binds it (README sections above, and
docs/hardware/), and the framing: one PACKET_MSG per frame out; every PACKET_MSG of a transfer in, checked against the transfer and handed up one per request.

## Wire format

NNET requests carry the 20-byte header (`NNET` magic, version 1, op, flags,
request id, payload length); replies repeat it and add a 4-byte signed status:

```text
magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32
```

## State ownership

`driver.rndis0` owns the bound device's bulk pipes, its MAC, the RNDIS request id, and the frames of the last transfer received. driver.xhci0 owns slots, endpoint contexts,
rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

- Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core.
- Do not add hardware authority here.
- Bound every length the device sends before using it.
- Log one line per step a bind can stop at.

## Release target

```text
USB device -> driver.xhci0 -> driver.rndis0 -> net.core
```

DHCP and a web page over a real RNDIS device.

## Release evidence

Build and host-proof evidence only so far (rndis_proofs, 22 tests).
Runtime evidence needs driver.xhci0's polled bulk IN and control OUT data
stage (docs/hardware/usb-net.md), the kernel spawn, and a real device: none
is on main yet.

## Release checklist

- Capsule builds with zero warnings; clippy clean.
- rndis_proofs passes.
- driver.xhci0 serves OP_BULK_IN_POLL and control OUT data stages.
- The kernel spawns the capsule and holds its endpoint.
- On hardware: `[usbnet rndis] port N up, mac ...` and
  `[NET-CORE] bind: interface up on driver.rndis0`, then DHCP and a page.

## Explicit non-goals today

ActiveSync devices, KEEPALIVE answers, wireless RNDIS (rndis_wlan), media state tracking, devices whose MaxTransferSize is under one frame.

## Verification

`cd userland/rndis_proofs && cargo test` (22 pass at the time of
writing); the capsule's `cargo build --release` and `cargo clippy` for
x86_64-nonos-user. Not verified under QEMU or on hardware.
