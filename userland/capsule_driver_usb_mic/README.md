# capsule_driver_usb_mic

Status: wired, not yet signed. The kernel mirror
(`src/hardware/usb_mic_capsule`) embeds and spawns it under
`nonos-capsule-driver-usb-mic` wherever the inventory sees a USB host,

## Role

`driver.usb_mic0` captures from a USB Audio 1.0 or 2.0 microphone (a
headset's, a webcam's, a standalone one) on any USB host's port, root or
virtual. The kernel never sees a microphone, so this driver is the gate:
sound goes only to a peer that holds MicrophoneCapture, while the
compositor shows that the microphone is on.

```text
peer --OPEN/START--> usb_mic0 --MkCapCheck(peer, MicrophoneCapture)--> kernel
usb_mic0 --NCMP 0x000A kind 2--> compositor (status 0, or no stream)
usb_mic0 --SET_INTERFACE, rate--> host --> device
usb_mic0 --OP_ISOCH_IN batches--> xhci0 --> ring --> peer READ
```

It shares its class code with `driver.usb_audio0` (`uac/`, `rate.rs`,
`xhci/portsc.rs`, `xhci/speed.rs`), and the isoch batch reader and the
indicator notice with `driver.uvc0` (`batch.rs`, `indicator.rs`), each
mounted by path. Its own are the gate (`src/gate.rs`), the capture ring
(`src/ring.rs`), the wire (`src/wire.rs`, `src/frame.rs`, `src/ops.rs`,
`src/reads.rs`) and the stream (`src/begin.rs`, `src/start.rs`,
`src/stream.rs`, `src/xhci/isoch.rs`).

## Microkernel contract

- `MkServiceLookup` finds each USB host and the compositor; every request
  to the device is an IPC call to its host.
- `MkIpcRecvFrom` serves `service:4520:driver.usb_mic0`; `MkIpcReply`
  answers. The endpoint is not held: the bit is the gate.
- `MkCapCheck(peer, MicrophoneCapture)` at every OPEN and START.
- `MkMmap` for the heap; `MkDebug` for the `[usb-mic]` lines, when granted.

## Interface contract

NMIC version 1: the host wire's 20-byte header with magic `NMIC`; a
reply's payload starts with a 4-byte status. Samples are 48 kHz signed
16-bit little-endian, channels interleaved.

- `HEALTHCHECK` (1): 0.
- `OPEN` (2): `vendor u16, product u16, channels u8, 0, 0, 0, rate u32`.
  -1 without MicrophoneCapture, -19 with no microphone.
- `START` (3): `channels u8, 0, 0, 0, rate u32`. -1 without the bit, or
  when the compositor does not answer its indicator with 0; -16 while
  another peer's stream runs; -22 on a host without isochronous IN.
- `READ` (4) `most u16`: `overrun u32` (bytes lost since the last read),
  then up to `most` (at most 4000) bytes of whole frames.
- `STOP` (5): 0. READ and STOP are the streaming peer's alone.

## Wire format

As above; every field little-endian. To the host it speaks the host wire
(`NXHC`): CONTROL_TRANSFER, PORT_STATUS, and on `driver.xhci0`
OP_CONFIGURE_ISOCH (0x18) and OP_ISOCH_IN (0x1A).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18`: IPC, Memory. Debug is optional. No
Driver, Mmio, Irq or Dma: it reaches the microphone only through a host.
It asks about MicrophoneCapture; it never holds it.

## Current implemented surface

- The first capture alternate setting: an isochronous IN data endpoint and
  no OUT one, PCM, 16 bits in 2-byte subslots, one or two channels. 1.0
  lists 48 kHz; 2.0 has its terminal's clock set and read back at 48000.
- One stream, one streaming peer at a time.
- Each batch is the packets of about 8 ms; a device that would not leave
  room for a frame more per packet (an asynchronous one catching up) is
  refused at bind.

## State ownership

The capsule owns the device's slot from bind until it leaves, the
endpoint's bandwidth while a stream runs, the capture ring and the
session (which peer streams).

## Operating rules

The kernel is asked about the bit at every OPEN and START; the answer is
never cached. No stream runs without the compositor's indicator. The
capture interface sits at alternate 0, taking no bandwidth, whenever no
stream runs.

## Privacy and persistence

Captured sound is kept only in memory, at most 64 KiB (about a third of a
second of stereo), only while a stream runs, and only for the peer that
started it; a new START begins an empty ring. Nothing is written anywhere.

## Runtime lifecycle

Spawned once, after the USB hosts and usb_hub0. It looks for a microphone
every second until one binds, then answers its inbox; while a stream runs
each batch call waits on the one before, which paces the loop.

## Failure model

A microphone that leaves ends the stream (the compositor told) and gives
its slot back; the next search finds it again. Three failed batches in a
row end the stream; the peer's next READ is refused (-1) and it may START
again. A rate the device will not take refuses START and leaves the
alternate at 0.

## Release target

Headset and webcam microphones and USB microphones on xHCI, captured for

## Release evidence

`usb_mic_proofs` covers the capture alternate in 1.0 and 2.0 descriptors
(and that a playback alternate's sync or feedback endpoint is none), the
gate, the ring and the framing. The shared class code is also covered by
`usb_audio_proofs`, the indicator notice and batch reader by `uvc_proofs`.
`kernel_proofs` covers the spawn and the held table. No boot yet.

## Release checklist

- Decision 9: which capsule may hold MicrophoneCapture.

## Real hardware bring-up checklist

- A full-speed USB headset's mono microphone: a READ loop hears speech
  with no overrun.
- A high-speed USB Audio 2.0 microphone with a programmable clock.
- The indicator on while capturing and off after STOP or unplug.

## Explicit non-goals today

No rates but 48 kHz, no 24-bit, no more than two channels, no volume or
mute controls, no feedback endpoint, no shared-memory path (a read moves
4000 bytes at most), and no isochronous IN on EHCI, OHCI or UHCI.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/usb_mic_proofs && cargo test --release)`, with
`usb_audio_proofs`, `uvc_proofs` and `kernel_proofs`.
