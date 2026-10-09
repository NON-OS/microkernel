# nonos_avi

`nonos_avi` is a `no_std` RIFF/AVI demuxer with no dependencies. It reads
the header list and the `idx1` index and hands back where each video frame
lies in the file; it decodes nothing. The video player
(`userland/capsule_video_player`) is its one user and decodes the frames
itself with `zune-jpeg`.

## API

- `AviFile::parse(bytes)` checks `RIFF` and `AVI `, walks the top-level
  chunks, parses the `hdrl` list (`parse_hdrl`), notes where `movi`'s data
  starts, and decodes `idx1` into frame offsets. The result carries the
  main header (`AviHeader`), the first video stream's `VideoInfo` (rate,
  scale, codec fourcc, width, height) and the `index`, a list of
  `FrameRef { offset, len }`. `fps_milli` gives the frame rate times
  1000.
- `parse_idx1` keeps only the entries of the video stream and reserves its
  list fallibly, so an index too large for the heap is `TooManyFrames`
  rather than an abort.
- `chunks(buf)` iterates RIFF chunks and stops at the first chunk that runs
  past the buffer.
- The building blocks are public too: `parse_avih`, `is_video_strh`,
  `parse_strh_rate`, `parse_strf_video`, and `u16_at`, `u32_at`,
  `fourcc_at`, which return `Truncated` instead of panicking on a short
  buffer.

Errors are `AviError`: `Truncated`, `NotRiff`, `NotAvi`, `MissingHeader`,
`NoVideoStream`, `UnsupportedCodec`, `NoFrames`, `NoIndex`,
`TooManyFrames`.

## What it does not do

- It needs the `idx1` index, which AVI keeps after `movi`. The video player
  parses only the first 2 MiB of a file, so a file whose index lies past
  that fails to open.
- No OpenDML (`indx`) indexes, no audio streams, no other containers.
- The codec fourcc is reported, not checked; the player decides what it
  can decode.

## Tests

`tests/parse_real.rs` parses the player's `assets/clip.avi`, and
`tests/hostile.rs` feeds the parser malformed files. Run `cargo test` in
this directory. Video playback is described in
[Audio and media](../../docs/handbook/audio-and-media.md).
