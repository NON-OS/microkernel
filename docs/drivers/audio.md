# Audio: Intel HD Audio

What plays sound in NONOS 0.9.2: the HD Audio driver, the audio server above it, the volume keys, and the machines it cannot play on, with the reason.

## What plays

- One output stream, 48 kHz, 16-bit, stereo (`PLAYBACK`, `userland/capsule_driver_hda/src/controller/codec/format.rs:59-62`).
- Through the analog speaker, headphone and line-out pins the codec's pin configuration names, speakers first, up to six that have a path to a converter (`plan`, `userland/capsule_driver_hda/src/controller/codec/plan.rs:80-126`; `MAX_OUTPUTS`, `userland/capsule_driver_hda/src/controller/codec/plan.rs:32`).
- The headphone jack is read twice a second. With headphones in, the speakers are switched off; when they come out, the speakers come back (`POLL_MS`, `userland/capsule_driver_hda/src/server/runner/jack_poll.rs:29`).
- One master volume from 0 to 100 with a mute switch, which the volume keys step.

Nothing plays at boot. The ring starts silent and the codec's outputs stay muted until a player opens a stream (`RING_BYTES`, `userland/capsule_driver_hda/src/setup/sequence.rs:136-144`).
