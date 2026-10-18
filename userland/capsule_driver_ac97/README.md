# capsule_driver_ac97

## Role

`capsule_driver_ac97` drives the Intel 82801AA (ICH) AC'97 audio controller
(PCI 8086:2415) and the codec on its link: QEMU's `AC97` and VirtualBox's
AC'97. It plays what the audio server mixes, at 48 kHz, 16-bit stereo. The
audio server reaches it exactly as it reaches the HD Audio driver.

```text
audio.server --NHDA (write PCM, start, stop, output status)--> driver.ac97_0
                                                                   |
                         queue -> four 8 KiB periods in DMA <- refill on the
                         engine's position; 32 descriptors loop over them
```

## Microkernel contract

- `MkDeviceList` finds the controller: 8086:2415 with I/O BARs 0 (mixer,
  at least 0x80 ports) and 1 (bus master, at least 0x38 ports).
- `MkDeviceClaim`; `MkPciConfigWrite` sets Bus Master and Interrupt Disable.
- `MkPioGrant` on BAR0 and BAR1: every register access is a kernel `in` or
  `out` through the grant.
- `MkDmaMap` once, coherent and below 4 GiB: the descriptor list in the
  first page, the 32 KiB sample ring after it.
- No `MkMmioMap`, no `MkIrqBind`: the driver polls the engine's position.
- `MkIpcRecv` serves `service:4282:driver.ac97_0`; the kernel holds the
  endpoint to `audio.server` alone.

## Interface contract

The PCM sink wire `driver.hda0` serves (`NHDA`, magic 0x4E484441, version 1,
20-byte header), so `audio.server` drives either with the same code:

| Op | Value | Meaning | Reply |
|---|---|---|---|
| `OP_HEALTHCHECK` | 1 | up | status |
| `OP_WRITE_PCM` | 7 | queue up to 4096 bytes of 48 kHz s16le stereo | status, `-11` while full |
| `OP_STREAM_START` | 8 | ring silenced, engine started, outputs opened | status |
| `OP_STREAM_STOP` | 9 | outputs muted, engine stopped, queue emptied | status |
| `OP_OUTPUT_STATUS` | 10 | ready, the codec's vendor id, line out | status + 12 bytes |

A payload on any op but 7, a short or long write, or any other op is
refused with `-22`. A frame that is not the wire is refused under the op
and id it names.

## Authority

`IPC`, `Memory`, `Driver`, `DeviceEnum`, `Dma`, `Pio`
(`CAPSULE_REQUIRED_CAPS = 0x198018`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Mmio`, `Irq` or `Network`.

```text
allowed:   the ICH AC'97 claim, its two port windows, one DMA grant
forbidden: any other device, capture, network, files
```

## Privacy and persistence

Playback only: the driver never starts the PCM in or microphone engines,
so nothing is recorded. Samples live in the queue and the ring until played.

## Runtime lifecycle

Claim, command bits, both port grants, the DMA grant; the PCM out engine
stopped in case firmware left it running; the link cold reset with
interrupts off and two channels, then the primary codec's ready bit (a
second); the codec reset and its DAC, mixer and Vref ready (half a
second); variable rate off, so the DAC runs at the link's 48 kHz; outputs
muted; the 32 descriptors written, entry `i` on period `i % 4`, buffer
underrun policy "zeros", no completion interrupt. Serve: between requests
the engine's current index is read, the last valid index pulled up to
trail it (the list never runs out while the driver looks), and every period
played since the last look refilled from the queue.

## Failure model

Every bring-up step names its failure and gives back the DMA grant, both
port grants and the claim, newest first. A window that is not decoded
reads all ones and is refused at the codec's vendor id. A box that never
finishes its register reset refuses the bring-up, or the start (outputs
left muted). A driver held off longer than the list lasts finds the engine
halted on its last valid entry; the next look starts it again. A queue that
runs dry plays silence and is counted as one underrun. Bring-up is tried a
bounded number of times (`nonos_libc::start_driver`).

## Current implemented surface

- One output stream, 48 kHz, 16-bit stereo, line out at full level (the
  audio server applies the user's volume to the samples).
- Polled position, four periods of about 43 ms, three ahead of the engine.

## Wire format

The `NHDA` header as `driver.hda0` reads it; replies echo op, flags and
request id, then an `i32` status; `OP_OUTPUT_STATUS` adds `u32 verdict,
u16 codec_vendor, u16 codec_device, u8 outputs, u8 plugged, u16 reserved`.

## State ownership

The capsule owns the controller, the codec's registers, the list and the
ring. The audio server owns the mix and the volume.

## Operating rules

- Mute before the engine stops; open only once it runs.
- Never ask the engine for an interrupt: the driver polls.
- Never touch the PCM in or microphone boxes.

## Release target

Sound in QEMU (`-device AC97`) and VirtualBox (AC'97 controller) through
the same player and system sounds HD Audio machines use.

## Release evidence

`ac97_proofs` runs the shipping source, port windows and mixer semaphore
included, against a model of the controller and a STAC9700 codec as QEMU's
ac97 answers: the order of the grants, the cold reset, 48 kHz, the list,
every refusal and what it gives back, a stream played byte for byte after
its silence, the list wrapping three times without a halt, a late look
restarting a halted engine, underruns, stop muting first, and the audio
server's own wire against the driver. A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_ac97` keys).
- `ac97_proofs` and `audio_proto_proofs` pass.
- QEMU `-device AC97` and VirtualBox AC'97 play the player's files and the
  desktop's sounds; the log names `driver.ac97_0` as the output.

## Explicit non-goals today

No capture, no S/PDIF, no 4 or 6 channel output, no rates other than 48 kHz,
no later ICH or other vendors' AC'97 controllers, no hardware volume.

## Verification

- Build: `make -B nonos-mk-driver-ac97`
- Proofs: `cd userland/ac97_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
