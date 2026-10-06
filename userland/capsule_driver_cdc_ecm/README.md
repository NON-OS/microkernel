# capsule_driver_cdc_ecm

## Role

`capsule_driver_cdc_ecm` is the USB CDC Ethernet Control Model class capsule.
It finds one ECM function on driver.xhci0's ports, binds it as Linux
`cdc_ether` does, and serves Ethernet frames to net.core over NNET, the
protocol every NONOS network driver speaks.

```text
USB Ethernet adapter or phone (ECM)
        |
        v
driver.xhci0 -- descriptors, control, bulk OUT, polled bulk IN
        |
        v
driver.cdc_ecm0 -- one frame per transfer --> net.core
```

The search, the xHCI client and the NNET service are `nonos_usbnet`'s; this
capsule holds only the ECM binding and framing (`src/ecm`).

## Binding

1. The first configuration with a communications interface of subclass 0x06,
   a Union naming its data interface, and an Ethernet Networking functional
   descriptor. QEMU's usb-net offers RNDIS first and ECM second; ECM is taken.
2. The bulk pipes go to driver.xhci0, then SET_CONFIGURATION, then
   SET_INTERFACE to the data interface's alternate setting with the pipes.
3. The station address from the iMACAddress string.
4. SET_ETHERNET_PACKET_FILTER: directed, broadcast, all multicast. A STALL is
   ignored, as Linux ignores it.

RTL8153 adapters (0bda:8153, 17ef:721e) are left to driver_rtl8153, as Linux
leaves them to r8152.

## Framing

One frame per bulk transfer. A frame that fills its last packet gets one
padding byte (Linux `usbnet_start_xmit`). A bulk IN is 2048 bytes; a runt
under 14 bytes is dropped, and a frame past 1514 bytes is cut to 1514.

The link reads up once bound: the network notifications arrive on an
interrupt endpoint, and driver.xhci0 does not configure one beside bulk pipes.

## Log

`log usbnet` shows `[usbnet cdc-ecm]` lines: each port tried and why it was
not taken, `port N up, mac ...` when bound, and `device stopped answering`
when it is given back.

## Contract

Caps `0x200018` (IPC, Memory, Debug). Service `driver.cdc_ecm0` on port 4250,
reply port 4251. Proofs: `userland/cdc_ecm_proofs`.

## Microkernel contract

The manifest grants `IPC`, `Memory` and `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0x200018
```

The capsule receives requests with `MkIpcRecvFrom` and answers with
`MkIpcReply`, finds driver.xhci0 with `MkServiceLookup` and calls it with
`MkIpcCall` (bounded by a timeout), writes its `[usbnet cdc-ecm]` lines with
`MkDebug`. It makes no enumeration, MMIO, IRQ, DMA
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
allowed:   descriptor classification, CDC-ECM binding and framing, frame counters
forbidden: xHCI ownership, USB scheduling, DMA buffers, routing, sockets, IP policy
```

The kernel holds `driver.cdc_ecm0` to the wired stack (net.core, net.l2), so no
app reaches it.

## Privacy and persistence

Frames pass through and are not kept past the request that carries them. The
capsule stores no product strings, serial numbers or frame contents; it holds
the bound device's bulk pipes, its MAC, and one receive and one send buffer in process memory, dropped when the process ends.

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

CDC-ECM binding as the Linux driver binds it (README sections above, and
docs/hardware/), and the framing: one Ethernet frame per bulk transfer, a padding byte when a frame fills its last packet.

## Wire format

NNET requests carry the 20-byte header (`NNET` magic, version 1, op, flags,
request id, payload length); replies repeat it and add a 4-byte signed status:

```text
magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32
```

## State ownership

`driver.cdc_ecm0` owns the bound device's bulk pipes, its MAC, and one receive and one send buffer. driver.xhci0 owns slots, endpoint contexts,
rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

- Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core.
- Do not add hardware authority here.
- Bound every length the device sends before using it.
- Log one line per step a bind can stop at.

## Release target

```text
USB device -> driver.xhci0 -> driver.cdc_ecm0 -> net.core
```

DHCP and a web page over a real CDC-ECM device.

## Release evidence

Build and host-proof evidence only so far (cdc_ecm_proofs, 7 tests).
Runtime evidence needs driver.xhci0's polled bulk IN and control OUT data
stage (docs/hardware/usb-net.md), the kernel spawn, and a real device: none
is on main yet.

## Release checklist

- Capsule builds with zero warnings; clippy clean.
- cdc_ecm_proofs passes.
- driver.xhci0 serves OP_BULK_IN_POLL and control OUT data stages.
- The kernel spawns the capsule and holds its endpoint.
- On hardware: `[usbnet cdc-ecm] port N up, mac ...` and
  `[NET-CORE] bind: interface up on driver.cdc_ecm0`, then DHCP and a page.

## Explicit non-goals today

NETWORK_CONNECTION and speed notifications (no interrupt endpoint beside the bulk pipes), multicast filter lists, more than one device per capsule, jumbo frames.

## Verification

`cd userland/cdc_ecm_proofs && cargo test` (7 pass at the time of
writing); the capsule's `cargo build --release` and `cargo clippy` for
x86_64-nonos-user. Not verified under QEMU or on hardware.
