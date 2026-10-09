# audio_proto_proofs

Host proofs for the audio service's wire format, `nonos_audio_proto`. The
server parses requests by fixed offset, so the format the clients build
is checked here against what the server reads.

| Tests | What they hold |
|---|---|
| `header_tests` | the 20-byte header round-trips |
| `tone_tests` | the 12-byte tone payload: frequency, duration, gain |
| `refusal_tests` | a frame the server refuses is answered under the op and request id it named, or zeros when too short |
| `stream_owner_tests` | a stream answers only the client that opened it |

`src/server/` holds the server side the tests decode with.

Run: `cargo test` in this directory. The audio path is described in
[Audio and media](../../docs/handbook/audio-and-media.md).
