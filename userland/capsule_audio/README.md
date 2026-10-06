# capsule_audio

`audio_server` is the system audio service registered as `audio.server`. Client capsules (the
audio player is one, through `userland/capsule_audio_player/src/audio_client/`) send it tones, PCM
blocks or stream feeds; it mixes them additively into signed 16-bit stereo and forwards the result
to the `driver.hda0` PCM sink over IPC. It owns no hardware. The handbook page is
[Audio and media](../../docs/handbook/audio-and-media.md).

## Role

A `no_std` service (`src/main.rs` calls `server::run`). The kernel spawns it from
`src/hardware/audio_capsule/spawn.rs` under the feature `nonos-capsule-audio`, requesting
`IPC | Memory`, and `Debug` when it compiles `capsule-serial-debug`. At start `src/server/run.rs` resolves the sink with up to 100 lookups of
`driver.hda0`, 20 ms apart, about two seconds in all (`src/sink/client.rs`), then serves its inbox with a 5 ms receive timeout and runs
the stream pump whenever the inbox is idle.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x18`, commented in `Capsule.mk` as `IPC | Memory`, and
`CAPSULE_OPTIONAL_CAPS := 0x100`:

- `0x008` IPC: receive client requests, call the `driver.hda0` sink.
- `0x010` Memory: heap for the stream rings.
- `0x100` Debug, optional: the `[AUDIO]` serial markers written by `src/mark.rs`; only a `capsule-serial-debug` build grants it,
  and without it the markers are dropped.

Endpoints: service `service:4872:audio.server`, reply `reply:4873:endpoint.4294967321`. Replies are
sent to `0x1_0000_0019` (`KERNEL_REPLY_ENDPOINT` in `src/server/dispatch.rs`), which is that inbox.

## Interface

The wire format comes from the shared crate `userland/audio_proto`; `src/server/proto.rs` decodes
the header and `src/server/dispatch.rs` routes by op:

| Op | Handler (`src/server/ops.rs`) |
|---|---|
| `OP_PLAY_TONE` | synthesize a square wave (`src/server/tone.rs`), mix, forward |
| `OP_PLAY_PCM` | add up to 1024 stereo frames to the mixer, forward |
| `OP_STOP` | clear the mixer |
| `OP_STREAM_OPEN` | open one of four stream slots, reply with its id |
| `OP_FEED_PCM` | push frames into the slot's ring, then run the pump |
| `OP_PAUSE`, `OP_RESUME`, `OP_CLOSE` | per-stream control |
| `OP_OUTPUT_STATUS` | ask driver.hda0 whether the machine plays, and why not |

Each reply is the header plus an `i32` status (`E_OK`, `E_AGAIN`, `E_INVAL`,
`E_NODEV`). `OP_STREAM_OPEN` is refused with `E_NODEV` when the driver says
this machine cannot play (Intel SOF, AMD ACP, HDMI only, no codec) or there
is no driver at all, and `OP_OUTPUT_STATUS` then says which, as a code the
player and Settings turn into a sentence (`nonos_audio_proto::output`). A frame the
decoder refuses is answered with `E_INVAL` under the op and request id it named, or zeros when it
is too short to name them (`refuse` in `src/server/dispatch.rs`). The pump in
`src/server/pump.rs` mixes 1024-frame periods from every unpaused stream, keeps up to three ahead
at the sink, and sends start and stop to the driver as streams appear and go
(`src/sink/wire.rs`, ops 7, 8 and 9).

## State and privacy

All state is in memory: the one-shot mixer, a `StreamTable` of four slots with a 16384-sample ring
each (`src/server/streams.rs`, `src/server/ring.rs`), and the pump state. A stream answers only
the client that opened it: feed, pause, resume and close from another client answer `E_INVAL`, as
for an unknown id. One client holds at most two of the four slots, and every two seconds the
streams of a client that ended are closed (held by `userland/audio_proto_proofs`). Audio is not recorded or
stored, and the capsule has no FileSystem, Network or hardware bits.

## Build and test

- `make nonos-mk-audio` and `make nonos-mk-audio-sign`; `nonos-mk-driver-hda-smoketest-test` and
  `nonos-mk-audio-player-smoketest-test` in `mk/20-build.mk` both sign it into their kernels.
- The `nonos-audio-smoketest` feature, which only `nonos-mk-driver-hda-smoketest-test` turns on,
  plays the self-tests (two tones, a two stream mix, a 4096-byte square tone) on start and logs
  `[AUDIO] sink-ok` or `[AUDIO] sink-fail`. A shipped image is built without it and plays
  nothing at boot; it logs `[AUDIO] output driver.hda0`, or `[AUDIO] no output device, playback
  refused` when no sink resolves.
- `userland/audio_proto_proofs` checks the shared wire format, the refusal replies and stream
  ownership on the host.
- `tests/host/` holds small programs that include `ring.rs`, `streams.rs`, `proto.rs` and
  `sink/wire.rs` through `#[path]` (see below).

## Not done yet

- If no sink resolves, the server stays up, answers `OP_OUTPUT_STATUS` with "no sound hardware",
  refuses a stream open with `E_NODEV` and anything else with `E_INVAL` (`src/server/dispatch.rs`),
  but there is no later retry of the lookup.
- `OP_PLAY_TONE` and `OP_PLAY_PCM` add into the shared mixer without clearing it
  (`src/server/ops.rs`); only `OP_STOP` resets it, so successive one-shot plays stack.
- The `tests/host/` programs are plain `fn main` files in a subdirectory, which cargo does not
  discover; no `[[test]]` entry or script in the repo runs them.
