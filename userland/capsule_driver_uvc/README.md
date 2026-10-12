# capsule_driver_uvc

Status: built and proven on the host, not yet wired. The kernel mirror,
spawn plan and service port wait on the port numbers for `driver.uvc0`,
so today nothing streams.

## Role

`driver.uvc0` drives a USB Video Class camera on any USB host's port,
root or virtual (behind a hub). It claims the camera's video streaming
interface, negotiates a format with PROBE and COMMIT, reads isochronous
IN payloads through `driver.xhci0`, and assembles them into whole frames.
The kernel never sees a camera, so this driver is the gate: a frame goes
only to a peer that holds CameraCapture.

```text
peer --OPEN/START--> uvc0 --MkCapCheck(peer, CameraCapture)--> kernel
uvc0 --PROBE/COMMIT, SET_INTERFACE--> host --> camera
uvc0 --OP_ISOCH_IN batches--> xhci0 --> payloads --> whole frame
peer --READ_FRAME--> uvc0 (4000-byte pieces); uvc0 --NCMP 0x000A--> compositor
```

The class parsing, the probe, the alternate setting choice and the frame
assembly are `nonos_uvc`'s. This capsule holds the gate (`src/gate.rs`),
the camera wire (`src/wire.rs`, `src/ops.rs`, `src/listing.rs`), the
isochronous batches (`src/isoch.rs`, `src/batch.rs`, `src/stream.rs`) and
the compositor notice (`src/notify.rs`).

## Microkernel contract

- `MkServiceLookup` finds each USB host and the compositor; every request
  to a device is an IPC call to its host.
- `MkIpcRecvFrom` serves the camera wire; `MkIpcReply` answers.
- `MkCapCheck(peer, CameraCapture)` at every OP_OPEN and OP_START.
- `MkMmap` for the heap; `MkDebug` for the `[uvc]` lines, when granted.

## Interface contract

NUVC version 1: the host wire's 20-byte header with magic `NUVC`; a
reply's payload starts with a 4-byte status.

- `HEALTHCHECK` (1): 0.
- `OPEN` (2): the camera's ids, formats and frame sizes. -1 without
  CameraCapture, -19 with no camera.
- `START` (3) `format u8, frame u8, 0, 0, interval u32`: the most bytes a
  frame takes. -1 without the bit, -16 while another peer streams, -22
  on a host without isochronous IN.
- `READ_FRAME` (4) `offset u32`: `sequence, total, offset`, then up to
  4000 bytes of the last whole frame. -11 before the first frame.
- `STOP` (5): 0. READ_FRAME and STOP are the streaming peer's alone.

## Wire format

The camera wire above. To the host it speaks the host wire (`NXHC`):
ENABLE_SLOT, ADDRESS_DEVICE, CONTROL_TRANSFER, DISABLE_SLOT, and on
`driver.xhci0` OP_CONFIGURE_ISOCH (0x18) and OP_ISOCH_IN (0x1A). Every
field is little-endian.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18`: IPC, Memory. Debug is optional. No
Driver, Mmio, Irq or Dma: it reaches the camera only through a host. It
asks about CameraCapture; it never holds it.

## Current implemented surface

- Uncompressed (YUY2, NV12) and MJPEG formats, every frame size listed.
- The smallest alternate setting whose bandwidth fits the committed
  payload size.
- Isochronous IN on `driver.xhci0` only (ops 0x18 and 0x1A); EHCI, OHCI
  and UHCI cameras are listed but answer -22 at START.
- The compositor told when streaming starts and stops (NCMP op 0x000A),
  for the indicator.

## State ownership

The capsule owns the camera's slot from its claim until the camera
leaves, the streaming endpoint's bandwidth while a stream runs, and the
one session: which peer streams and how many frames it has had.

## Operating rules

One camera, one stream, one streaming peer at a time. The kernel is asked
about the bit at every OPEN and START; the answer is never cached. The
first claim of a port wins, so a camera another driver holds is left
alone.

## Privacy and persistence

Only the last whole frame is held, in memory, and only while a stream
runs; STOP, a failed stream or a camera that leaves drops it. The bit
is asked again at every START, not at every read. Nothing is written
anywhere. No stream starts for a peer without the bit.

## Runtime lifecycle

Spawned once, after the USB hosts and usb_hub0. It looks for a camera
every second until one appears, then answers its inbox, and while a
stream runs it reads a batch between messages.

## Failure model

A camera that leaves ends the stream and gives its slot back; the next
search finds it again. Three failed batches in a row end the stream; the
peer's next READ_FRAME is refused (-1) and it may START again. A
compositor that does not answer leaves the stream as it is.

## Release target

Built-in laptop webcams and external USB 2 webcams, uncompressed and

## Release evidence

`uvc_proofs` covers the class descriptors of a modelled webcam, the
probe, the alternate setting choice, payload headers and frame assembly,
the batch reader, the gate and the listing. No boot yet.

## Release checklist

- Ports, kernel mirror and spawn plan for `driver.uvc0`.
- Decision 9: which capsule may hold CameraCapture.

## Real hardware bring-up checklist

- A laptop's internal webcam behind its internal hub: listed, a 640x480
  YUY2 stream and an MJPEG stream read whole.
- An external USB 2 webcam on an xHCI root port and behind a hub.
- The compositor's indicator on while it streams and off after STOP.

## Explicit non-goals today

No bulk-streaming cameras, no still capture, no camera controls
(exposure, focus), no H.264 or frame-based formats, no shared-memory
frames: a frame crosses the wire in 4000-byte pieces.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/uvc_proofs && cargo test --release)`, which covers
the descriptors, the probe, frame assembly, the batch reader, the gate
and the listing.
