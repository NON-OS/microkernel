# capsule_driver_usb_audio

Status: wired, not yet signed. The kernel mirror
(`src/hardware/usb_audio_capsule`) embeds and spawns it under
`nonos-capsule-driver-usb-audio` (the `microkernel-driver-usb-audio` build)
wherever the inventory sees an xHCI controller, before audio.server, which
plays through it while it reports a device ready and through `driver.hda0`
otherwise. It has no owner certificate yet, so no image carries it until the
owner signs it.

## Role

`capsule_driver_usb_audio` plays the audio server's stream on a USB Audio
Class 1.0 device: USB speakers, headsets and DACs. It finds the device on
driver.xhci0's ports, sets up its playback alternate setting, and serves
audio.server the NHDA PCM wire `driver.hda0` serves, so the server plays
through it with the sink client it has.

```text
audio.server --NHDA--> driver.usb_audio0 --isoch OUT--> driver.xhci0 --> device
```

The port search and the xHCI client are `nonos_usbnet`'s; this capsule holds
the class binding (`src/uac`, `src/bind.rs`), the isoch ops
(`src/xhci`), and the PCM service (`src/serve`, `src/wire`).

## Binding

1. The first configuration with an AudioStreaming interface (class 1,
   subclass 2, protocol 0) whose alternate setting is PCM (AS_GENERAL tag 1),
   Format Type I with 2 channels of 16 bits in 2-byte subframes at 48 kHz
   (listed, or inside a continuous range), and an isochronous OUT endpoint.
   Other alternates (24-bit, mono, 44.1 kHz only) and IN endpoints are passed
   over. A USB Audio 2.0 function (protocol 0x20) is not bound.
2. The port's speed from driver.xhci0's port status; a packet is the frames
   48 kHz plays in one service interval (192 bytes a 1 ms frame at full
   speed, 24 bytes a 125 us microframe at high speed with bInterval 1), and
   a device whose endpoint cannot carry that is refused.
3. SET_CONFIGURATION, SET_INTERFACE to the alternate, SET_CUR of the
   endpoint's sampling frequency to 48000 where its class endpoint
   descriptor says it takes one, then OP_CONFIGURE_ISOCH on driver.xhci0.
4. The Output Terminals in the AudioControl interface name what it plays
   into: a speaker, something worn (headphones, a headset), else a line out.

## Log

In a serial-debug build, `log usbnet` shows `[usbnet usb-audio]` lines (the
shared core's prefix):
each port tried and why it was not taken, `bound` with the device's ids,
and `device stopped answering` when it is given back.

## Microkernel contract

- `MkServiceLookup` finds `driver.xhci0`; every USB transfer is an IPC call
  to it.
- `MkIpcRecvFrom` serves `service:4246:driver.usb_audio0`; `MkIpcReply`
  answers.
- `MkServiceLookup` once for its own pid, to drop replies it sent itself;
  `MkMmap` for the heap.
- `MkDebug` for the log lines, when Debug is granted.

## Interface contract

NHDA, version 1, as `driver.hda0` frames it (its header, codec, errors and
output status are mounted, not copied):

- `HEALTHCHECK` (1): 0.
- `WRITE_PCM` (7): 1 to 4096 bytes of 48 kHz stereo s16le, queued whole;
  -11 while the 16 KiB queue cannot take it (the server's pump retries),
  -19 with no device.
- `STREAM_START` (8): -19 with no device. `STREAM_STOP` (9): stops and drops
  what was queued.
- `OUTPUT_STATUS` (10): verdict 0 with the device's vendor and product ids
  in the codec fields, the outputs (bit 0 speaker, 1 headphone, 2 line) and
  plugged for a worn device; verdict 1 with no device.

Other ops, and a payload on any op but WRITE_PCM, answer -22.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18`: IPC, Memory. Its own pid, which drops what
it sent itself, is read back through `MkServiceLookup`, so no CoreExec.
`CAPSULE_OPTIONAL_CAPS = 0x100`: Debug, in a serial-debug build only, for
the log lines. No Driver, DeviceEnum, Mmio, Irq, Dma or Pio: the controller,
its DMA and the stream's buffers are driver.xhci0's.

## Current implemented surface

- One device, one playback stream, 48 kHz stereo 16-bit only.
- Each batch is up to 8 ms of sound (at most 32 packets and 4032 bytes),
  sent with OP_ISOCH_OUT; the call returns when the batch before has played,
  which paces the loop. A stream that runs short plays silence.
- Three failed batches in a row give the device back and the search resumes.

## Wire format

As `driver.hda0`: a 20-byte NHDA header (`NHDA`, version, op, flags, request
id, payload length), then a 4-byte status and the op's payload.

## State ownership

The capsule owns its queue and its slot on driver.xhci0. The isoch ring and
its buffer are driver.xhci0's.

## Operating rules

No device access but through driver.xhci0. No reply to a request from
itself.

## Privacy and persistence

The queue holds at most 85 ms of what is playing; nothing is written
anywhere or kept past the process.

## Runtime lifecycle

Spawned once, where an xHCI controller is. It looks for a device every second until one binds, serves
requests between batches while the stream runs, and waits in the kernel
when it does not.

## Failure model

A port that fails to bind is retried up to three times, then left until its
device is replugged or a minute passes. A bound device that fails three
batches in a row is given back (its slot disabled) and the search resumes.
Requests are answered throughout, with -19 while there is no device.

## Release target

USB headsets and speakers on machines whose HD Audio is missing or not
supported (SOF laptops, AMD ACP), through audio.server.

## Release evidence

`usb_audio_proofs` on the host; the held-table proofs in `kernel_proofs`.
No boot yet.

## Release checklist

- Signed by the owner and added to the desktop image's features.
- A QEMU `-device usb-audio` boot playing through `driver.usb_audio0`.

## Real hardware bring-up checklist

- A full-speed headset (most are) and a high-speed DAC both bind and play.
- An asynchronous device plays without audible drift over a few minutes:
  no feedback endpoint is read, so its clock is not followed.
- Unplugging mid-stream gives the device back and replugging binds again.

## Explicit non-goals today

No capture, no volume or mute controls (the server mixes), no USB Audio 2.0
or 3.0, no feedback endpoint, no rates but 48 kHz, no more than two channels.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/usb_audio_proofs && cargo test --release)`.
Handbook: [drivers](../../docs/handbook/drivers.md).
