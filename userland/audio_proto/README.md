# audio_proto

`nonos_audio_proto` is the wire format between applications and the
`audio.server` capsule (`userland/capsule_audio`), written once so the
clients and the server cannot drift apart. `no_std`, no dependencies.

## Header

`write_header(out, op, request_id, payload_len)` lays out the 20-byte
(`HDR_LEN`) header, all fields little-endian:

| Bytes | Field |
|---|---|
| 0..4 | `MAGIC`, 0x4E41_5544 |
| 4..6 | `VERSION`, 1 |
| 6..8 | op |
| 8..12 | reserved, zero |
| 12..16 | request id |
| 16..20 | payload length |

A reply is the header followed by a 4-byte (`STATUS_LEN`) `i32` status:
`E_OK` (0), `E_AGAIN` (-11), `E_INVAL` (-22) or `E_NODEV` (-19).

## Ops

| Value | Op |
|---|---|
| 1 | `OP_PLAY_TONE` |
| 2 | `OP_PLAY_PCM` |
| 3 | `OP_STOP` |
| 4 | `OP_STREAM_OPEN` |
| 5 | `OP_FEED_PCM` |
| 6 | `OP_PAUSE` |
| 7 | `OP_CLOSE` |
| 8 | `OP_RESUME` |
| 9 | `OP_OUTPUT_STATUS` |

`OP_OUTPUT_STATUS` (`src/output.rs`) answers with an output code and flags:
whether this machine plays, and if not why (Intel SOF, AMD ACP, HDMI only,
no codec, no usable output, no sound hardware, driver not answering).
`output_message` and `output_short` give the words the player and Settings
show; a stream open on such a machine is refused with `E_NODEV` (-19).

`tone_request(out, request_id, hz, ms, gain)` builds a whole tone request:
the header and a 12-byte (`TONE_PAYLOAD_LEN`) payload of frequency,
duration and gain, `TONE_MSG_LEN` bytes in all. It writes nothing and
returns zero when `out` is too short.

## Users

`capsule_audio` (the server), `capsule_audio_player`, and
`capsule_desktop_shell`, whose UI sounds are tone requests
(`src/sound/play.rs`). `userland/audio_proto_proofs` checks the header and
tone layout against the offsets the server reads, and the refusal and
stream-owner rules. The audio path is described in
[Audio and media](../../docs/handbook/audio-and-media.md).
