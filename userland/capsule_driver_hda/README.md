# capsule_driver_hda

## Role

`capsule_driver_hda` is the Intel HD Audio controller capsule. It owns the PCI
HDA controller in userland: the register BAR, the CORB/RIRB verb rings, one
output stream with its buffer descriptor list, and the PCM ring the stream
plays from. It serves `driver.hda0` to the audio server.

```text
audio.server (the only sender the kernel admits)
    |
    | NHDA requests: info, tone, PCM chunks, stream start and stop
    v
driver.hda0 -- BAR0 MMIO, CORB/RIRB + BDL + PCM DMA --> HDA controller
    |
    `-- INTx, else one MSI-X vector, else polled
```

## Microkernel contract

The capsule uses only Mk and broker calls:

- `MkDeviceList` finds every PCI function of class 0x04, subclass 0x03, and
  Intel's subclass 0x01 (Skylake and later with the DSP enabled), with an MMIO
  BAR0, chipset controllers before graphics ones; it also notes an AMD audio
  coprocessor (class 0x0480) and an Intel SST engine (Haswell and Broadwell
  ULT ADSP 8086:9c36 and 9cb6, Atom LPE 0f28, 22a8 and 119a), which is never
  run as an HD Audio controller (`src/discover/`, `src/controller/sst.rs`).
- `MkDeviceClaim` gives this process exclusive controller ownership,
  `MkPciConfigWrite` turns on bus mastering and memory decoding, and
  `MkPciConfigRead` reads Intel's TCSEL, DEVC and CGCTL for the boot log.
- `MkMmioMap` maps the BAR0 register window.
- `MkIrqBind` binds the legacy line when firmware routed one, else one MSI-X
  vector, else the driver runs polled (`src/setup/irq.rs`,
  `src/setup/irq_plan.rs`). `MkIrqWait` and `MkIrqAck` serve it.
- `MkDmaMap` takes four grants: the CORB (1 KiB), the RIRB (2 KiB), the BDL
  (one page) and the 32 KiB PCM ring (`src/setup/dma.rs`).
- `MkIpcRecv` and `MkIpcSend` serve `driver.hda0` on
  `service:4218:driver.hda0`.

Only `audio.server` may send to `driver.hda0`: the kernel holds the endpoint
to it (`src/services/registry/held.rs`), by name and by pid. Applications talk
to the audio server, not to this driver.

The kernel keeps scheduling, isolation, capability checks, and grant teardown.
It does not mix audio, route streams, parse codec widgets, or hold user audio.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_INFO` | GCAP/GCTL/STATESTS snapshot | 28-byte controller record |
| `OP_CODEC_MASK` | detected codec slots | 8-byte mask payload |
| `OP_STREAM_LAYOUT` | GCAP-derived stream descriptor offsets | count plus 8-byte entries |
| `OP_CODEC_LIST` | codec vendor ids, asked over the CORB | count plus 8-byte entries |
| `OP_PLAY_TONE` | refill the PCM ring with the test tone and restart the stream | status word |
| `OP_WRITE_PCM` | queue up to 4096 bytes of PCM | status word, `E_AGAIN` (-11) when the queue is full |
| `OP_STREAM_START` | silence the ring and start the stream fed from the queue | status word |
| `OP_STREAM_STOP` | clear the stream's RUN bit and drop the queue | status word |
| `OP_OUTPUT_STATUS` | whether this machine plays, and if not, why | 12-byte output status |

On a machine it cannot play on the driver holds no hardware and answers only
`OP_HEALTHCHECK` and `OP_OUTPUT_STATUS`; every other op is `E_NODEV` (-19).

Only `OP_WRITE_PCM` takes a body; any other op with a body, or a header the
driver refuses, is answered `E_INVAL`.

## Authority

The manifest grants `IPC`, `Memory`, `DeviceEnum`, `Driver`, `Mmio`, `Irq`
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xF8018`). `Debug` (`0x100`) is optional:
only a `capsule-serial-debug` build grants it, and without it the `[HDA]`
marks are dropped.

```text
allowed:   HDA controller claim, BAR0 registers, IRQ, verb ring and stream DMA, IPC
forbidden: filesystem, mixer policy, microphone policy, admin
```

## Privacy and persistence

The driver plays what the audio server sends and records nothing. No input
stream is opened, so no microphone samples are read. PCM lives in the 64 KiB
queue in the capsule's heap and in the PCM ring, a DMA grant the broker zeroes
before it frees it. No samples or stream state are persisted, and no runtime
state survives process exit.

## Runtime lifecycle

One bring-up attempt (`src/setup/sequence.rs`) runs over each controller in
turn until one plays:

1. Claim the controller, turn on bus mastering and memory decoding, map BAR0,
   bind the interrupt, map the CORB and RIRB.
2. Stop every engine a previous owner left running, then a full reset: CRST
   low and seen low, held 1 ms, high and seen high, 2 ms for the codecs, then
   STATESTS (`src/controller/reset.rs`). Apollo Lake's DMA latency and the
   multi-link clock of Skylake-family parts are set as Linux sets them
   (`src/controller/intel.rs`).
3. Start the rings at the largest size the controller offers; a CORB read
   pointer reset that is not acknowledged is carried on from, as in Linux.
4. Ask every codec STATESTS names for its identity over the CORB, walk each
   audio function group (widgets, pin configurations, connection lists in
   short and long form with ranges, amp capabilities) and plan its outputs:
   every speaker, headphone and line out pin with a path to a DAC that plays
   48 kHz 16-bit (`src/controller/codec/`). The codec with speakers wins.
5. With no analog output, name the reason (`src/controller/verdict.rs`):
   an Intel SST engine on the bus, or an Intel DSP controller with no codec
   or only HDMI, needs SOF (logged `[HDA] intel sst dsp 8086:<id> found,
   needs SOF`, then `[HDA] no playable output: <reason>`), an AMD ACP
   on the bus explains an AMD machine, otherwise no codec, HDMI only or no
   usable output. The claims are given back and the driver serves only
   `OP_OUTPUT_STATUS` (`src/server/runner/status.rs`).
6. Otherwise map the BDL and the PCM ring, fill the ring with a test tone,
   power the codec to D0 and wait for it, apply Realtek's EAPD coefficient and
   `alc256_init` where Linux does, select each path's inputs, unmute each amp
   at its 0 dB step, enable the pins (speakers off while headphones are in),
   raise EAPD, drive an HP machine's codec GPIOs high, and join each DAC to
   the stream (`src/controller/codec/program.rs`).
7. Point the DMA position buffer at the BDL page on Intel, reset the first
   output stream descriptor with the full handshake and start it on four
   8 KiB periods (`src/controller/stream_run.rs`).

The serving loop (`src/server/runner/run.rs`) reads the headphone jack every
500 ms and switches the speakers with it, waits up to 50 ms for the
interrupt, then 5 ms for a request. On each buffer completion it either marks
`[HDA] play-complete` once (tone mode) or, while a stream started with
`OP_STREAM_START` is running, copies queued PCM into each period the
controller has finished, as read from the DMA position buffer on Intel or
LPIB elsewhere (`src/controller/position.rs`), and fills a period the queue
cannot cover with silence (`src/server/runner/refill.rs`).

Teardown is the handles' Drop: unmap, unbind and release the claim, which
takes the DMA grants with it.

## Failure model

Broker setup failure aborts the attempt and rolls back what it took. A
controller that does not leave reset, reads all ones, reports VMAJ or GCAP
zero, has no output stream, has DMA above 4 GiB without 64-bit addressing,
or whose codec does not reach D0 fails setup; a graphics card's controller
that fails is passed over. Every wait is bounded in milliseconds of uptime
(`src/controller/wait.rs`), each verb by 100 ms.

A machine with audio hardware this driver cannot play on is not a failure:
the driver names it (`OP_OUTPUT_STATUS`) and the audio server, the player
and Settings show the reason, for example "This laptop's audio needs Intel's
DSP firmware (SOF), which NONOS does not support".

With no HD Audio controller in the device list the capsule logs one line and
exits `EXIT_ABSENT` (2) before claiming anything. A controller that is present
but fails setup is retried on the shared bounded schedule
(`nonos_libc::bring_up`: seven tries, sleeping between them, each failed try
having released its claim); running out logs the last cause, prints the
numbered `[HDA] setup-fail` mark, and exits `EXIT_GAVE_UP` (6).

## Current implemented surface

- Controller discovery, claim, BAR0 map, bus mastering.
- INTx when firmware routed a line, else MSI-X, else polling. A line left at
  0xFF does not hide the controller.
- Full controller reset and codec discovery at every address.
- CORB/RIRB verb transport with its own RIRB read pointer; unsolicited
  responses are set aside.
- Codec walk, output planning for speakers, headphones and line outs through
  mixers and selectors, EAPD, Realtek EAPD coefficients and ALC256 init, HP
  codec GPIOs, headphone jack switching.
- Plain-words verdicts for SOF-only Intel, AMD ACP, HDMI-only and codec-less
  machines.
- One output stream: BDL, 32 KiB cyclic PCM ring, 48 kHz 16-bit stereo.
- A test tone at bring-up and on `OP_PLAY_TONE`.
- PCM playback from a 64 KiB queue fed by `OP_WRITE_PCM`, refilled per period.
- Input, output and bidirectional stream descriptor offsets over IPC.

## Wire format

Requests use the `NHDA` capsule header (magic 0x4E48_4441), version `1`, and
the shared 20-byte driver envelope. Replies start with a 4-byte status word.
Controller-info replies return a 28-byte fixed register snapshot. Codec-mask
replies return an 8-byte mask payload. Stream-layout replies return a 4-byte
count followed by 8-byte entries:

```text
u8 kind, u8 local_index, u16 global_index, u32 stream_descriptor_offset
```

Codec-list replies return a 4-byte count followed by 8-byte entries:

```text
u8 codec_address, u8 probe_ok, u16 vendor_id, u16 device_id, u16 reserved
```

An output-status reply returns a 12-byte body:

```text
u32 verdict, u16 codec_vendor, u16 codec_device, u8 outputs, u8 plugged, u16 reserved
```

`verdict` is 0 playing, 2 needs Intel SOF, 3 no codec, 4 HDMI only, 5 no
usable output, 6 AMD ACP; `outputs` has bit 0 speakers, bit 1 headphone jack,
bit 2 line out.

A write-PCM request carries up to `MAX_PCM_CHUNK` (4096) bytes of
little-endian 16-bit stereo samples after the header.

## State ownership

The capsule owns the BAR0 mapping, the IRQ grant, controller reset state, the
GCAP/GCTL snapshot, codec-presence mask, codec probe results, the CORB and
RIRB, the output path, the BDL, the PCM ring and queue, and the stream
descriptor it runs. Mixing, volume and routing belong to `audio.server`.

## Operating rules

- Keep mixer, policy routing, and permissions outside the controller driver.
- Do not persist samples or stream state.
- Any setup failure must unwind IRQ, MMIO, DMA and device claim.

## Release target

A signed audio-controller service with CORB/RIRB verb transport, codec
discovery, stream programming, BDL DMA, and playback driven by the audio
server. Capture and volume are not there yet.

## Release evidence

Proofs: `userland/hda_proofs`. No boot log for this driver is committed.
Release requires a QEMU `intel-hda` boot that plays, codec verb transport
validation on real silicon, IRQ completion proof, and teardown revocation
proof.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU HDA controller probe passes.
- Codec verb round trip works through CORB/RIRB.
- Playback produces completion interrupts and no underrun marks.
- Teardown proof shows no leaked MMIO/IRQ/DMA grants.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which only a `capsule-serial-debug` build grants, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- The controller leaves reset; the `[HDA] codecs mask=` line names every codec, and the `[HDA] path` lines name the speaker (0x14 on the HP 15s ALC236) and headphone (0x21) pins.
- The speakers play; plugging headphones in moves the sound to them (`[HDA] headphones in, speakers off`).
- On a DSP-only Intel laptop or an AMD ACP machine, Settings' Sound page and the player name the reason instead of staying silent.
- An `[HDA] intel pci: tcsel=` line means firmware left TCSEL or no-snoop set; the broker does not let the driver change them (see Explicit non-goals).
- The bring-up tone is heard, and PCM from the audio server plays without `[HDA] underrun` marks.
- On a laptop whose firmware leaves the interrupt line at 0xFF, the controller is still found; the driver asks for MSI-X, then polls, and still plays.

## Explicit non-goals today

No capture stream, mixer, volume policy beyond each amp's 0 dB step, HDMI
or DisplayPort output, Intel SOF or AMD ACP DSP audio, per-board Realtek
fixups beyond the HP GPIO case, more than one output stream, or persistent
audio state is implemented here. The PCI configuration writes Linux makes on
Intel (TCSEL, DEVC no-snoop, CGCTL clock gating around the reset) and AMD
(snoop at 0x42) need the broker's PCI write allowlist widened; until then
the driver reads and logs them and flushes its DMA buffers from the cache.

## Verification

- Build: `make -B nonos-mk-driver-hda`
- Proofs: `(cd userland/hda_proofs && cargo test --offline)`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Documentation check: the static gate requires this README and its authority,
  privacy, current surface, non-goal, and verification sections.
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [audio and media](../../docs/handbook/audio-and-media.md).
