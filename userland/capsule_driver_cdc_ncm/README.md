# capsule_driver_cdc_ncm

Status: not in the 0.9.2 image. The capsule builds and its host proofs
pass, but it has no owner certificate and the kernel never spawns it:
driver.xhci0 has no bulk IN poll or control OUT data stage yet.

## Role

`capsule_driver_cdc_ncm` is the USB CDC Network Control Model class capsule.
It finds one NCM function on driver.xhci0's ports, binds it as Linux
`cdc_ncm` does, and serves Ethernet frames to net.core over NNET, the
protocol every NONOS network driver speaks.

```text
phone tethering over NCM, or an NCM USB Ethernet adapter
        |
        v
driver.xhci0 -- descriptors, control, bulk OUT, polled bulk IN
        |
        v
driver.cdc_ncm0 -- NTB16 blocks <-> one frame per call --> net.core
```

The search, the xHCI client and the NNET service are `nonos_usbnet`'s; this
capsule holds only the NCM binding and NTB framing (`src/ncm`).

## Binding

1. The first configuration, of all the device offers, with a communications
   interface of subclass 0x0D and protocol 0, a Union naming its data
   interface, an Ethernet Networking and an NCM functional descriptor. A
   device with only ECM (subclass 0x06) is left to driver_cdc_ecm.
2. The bulk pipes go to driver.xhci0, then SET_CONFIGURATION, then
   SET_INTERFACE to the data interface's alternate 0.
3. GET_NTB_PARAMETERS (28 bytes, required); SET_CRC_MODE off when the device
   could add a CRC; SET_NTB_FORMAT 16-bit only when it also offers 32-bit.
4. A 10 ms pause, then SET_INTERFACE to the alternate with the bulk pipes.
5. The station address from the iMACAddress string.
6. SET_NTB_INPUT_SIZE to at most 4096 bytes (one bulk transfer) when the
   device offers another size; the 8-byte form when bmNetworkCapabilities
   bit 5 asks for it. GET/SET_MAX_DATAGRAM_SIZE when bit 3 is set.
7. SET_ETHERNET_PACKET_FILTER: directed, broadcast, all multicast. A STALL
   is ignored, as Linux ignores it.

## Framing

Out: one frame per NTB16, an NTH16, an NDP16 with one entry and its zero
entry, and the datagram placed on the device's wNdpOutAlignment,
wNdpOutDivisor and wNdpOutPayloadRemainder. A block that ends on a packet
boundary gets one more byte; one past min_tx_pkt is padded to the full NTB
size (not for DisplayLink docks), as `cdc_ncm_fill_tx_frame` does.

In: one bulk IN of the negotiated NTB input size; every NDP16 in the chain
and every datagram in it is checked against the transfer, queued, and
handed up one per call. A malformed block, NDP or entry is dropped.

The link reads up once bound: NETWORK_CONNECTION arrives on an interrupt
endpoint, and driver.xhci0 does not configure one beside bulk pipes.

## Log

`log usbnet` shows `[usbnet cdc-ncm]` lines: each port tried and why it was
not taken, the bind step that failed with its errno, `port N up, mac ...`
when bound, and `device stopped answering` when it is given back.

## Contract

Caps `0x200018` (IPC, Memory, Debug). Service `driver.cdc_ncm0` on port 4252,
reply port 4253. Proofs: `userland/cdc_ncm_proofs`. Notes:
`docs/hardware/cdc_ncm.md`.

## Microkernel contract

The manifest grants `IPC`, `Memory` and `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0x200018
```

The capsule receives requests with `MkIpcRecvFrom` and answers with
`MkIpcReply`, finds driver.xhci0 with `MkServiceLookup` and calls it with
`MkIpcCall` (bounded by a timeout), writes its `[usbnet cdc-ncm]` lines with
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
allowed:   descriptor classification, CDC-NCM binding and framing, frame counters
forbidden: xHCI ownership, USB scheduling, DMA buffers, routing, sockets, IP policy
```

The kernel holds `driver.cdc_ncm0` to the wired stack (net.core, net.l2), so no
app reaches it.

## Privacy and persistence

Frames pass through and are not kept past the request that carries them. The
capsule stores no product strings, serial numbers or frame contents; it holds
the bound device's bulk pipes, its MAC, the negotiated NTB limits, and the datagrams of the last NTB received in process memory, dropped when the process ends.

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

CDC-NCM binding as the Linux driver binds it (README sections above, and
docs/hardware/), and the framing: one datagram per 16-bit NTB out; every NDP and datagram of an NTB in, checked against the transfer and handed up one per request.

## Wire format

NNET requests carry the 20-byte header (`NNET` magic, version 1, op, flags,
request id, payload length); replies repeat it and add a 4-byte signed status:

```text
magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32
```

## State ownership

`driver.cdc_ncm0` owns the bound device's bulk pipes, its MAC, the negotiated NTB limits, and the datagrams of the last NTB received. driver.xhci0 owns slots, endpoint contexts,
rings and DMA; net.core owns addresses, routes and sockets.

## Operating rules

- Keep USB transfer mechanics in driver.xhci0 and IP policy in net.core.
- Do not add hardware authority here.
- Bound every length the device sends before using it.
- Log one line per step a bind can stop at.

## Release target

```text
USB device -> driver.xhci0 -> driver.cdc_ncm0 -> net.core
```

DHCP and a web page over a real CDC-NCM device.

## Release evidence

Build and host-proof evidence only so far (cdc_ncm_proofs, 33 tests).
Runtime evidence needs driver.xhci0's polled bulk IN and control OUT data
stage (docs/hardware/usb-net.md), the kernel spawn, and a real device: none
is on main yet.

## Release checklist

- Capsule builds with zero warnings; clippy clean.
- cdc_ncm_proofs passes.
- driver.xhci0 serves OP_BULK_IN_POLL and control OUT data stages.
- The kernel spawns the capsule and holds its endpoint.
- On hardware: `[usbnet cdc-ncm] port N up, mac ...` and
  `[NET-CORE] bind: interface up on driver.cdc_ncm0`, then DHCP and a page.

## Explicit non-goals today

32-bit NTBs, more than one datagram per NTB out, blocks over 4096 bytes, MBIM, link notifications (no interrupt endpoint beside the bulk pipes).

## Verification

`cd userland/cdc_ncm_proofs && cargo test` (33 pass at the time of
writing); the capsule's `cargo build --release` and `cargo clippy` for
x86_64-nonos-user. Not verified under QEMU or on hardware.
