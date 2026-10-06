# capsule_image_codec

`capsule_image_codec` is a userland decode service. A client puts an
image file's bytes in a surface it shares, sends the handle and the length,
and gets back a new shared ARGB8888 surface handle with the width, height,
stride and length. It decodes with the toolkit's PNG, BMP, GIF, JPEG and raw
LZ4 decoders, reads the image's dimensions first, and refuses an image over
16,777,216 pixels before it allocates (`src/server/handlers/decode_sized.rs`).
The image viewer is its client. The handbook page is
[Audio and media](../../docs/handbook/audio-and-media.md).

Service endpoint: `service:4412:image_codec`.
Reply endpoint: `reply:4413:endpoint.image_codec.reply`.

Ops:
- `OP_HEALTHCHECK`
- `OP_DECODE_PNG`
- `OP_DECODE_BMP`
- `OP_DECODE_LZ4_RAW`
- `OP_DECODE_JPEG`
- `OP_DECODE_GIF`

## Outputs

Each decode answers with a surface the client attaches and copies from. A client
has to attach it before its next decode request, within 30 s, and while it
runs. image_codec then lets the surface go: it unmaps it, and the kernel frees
the surface's slot once no client maps it. A client that attached in time keeps
its view until it releases. At most four outputs are held at once, the oldest
let go first (`src/server/outputs.rs`, proven by `userland/image_codec_proofs`).
Until this release nothing was let go, so every decoded image kept its memory,
plus one of the machine's 256 surface slots, for as long as the machine ran.

## Authority

Required caps `0x3018` (IPC | Memory | GraphicsSurfaceCreate |
GraphicsSurfaceMap). No driver caps.

## Errors

20-byte header with magic `0x474D_494E` ("NIMG" in little-endian byte
order), version 1. Malformed requests get a deterministic typed
errno, never a silent drop: `E_BAD_MAGIC` (wrong magic), `E_BAD_VERSION`
(version mismatch), `E_BAD_LEN` (too short, or not the declared length), `E_BAD_OP`
(unknown op), `E_INVAL` (bad body), `E_UNSUPPORTED` (codec/format not
supported), `E_NOMEM` (an image over the pixel limit, or the surface allocation failed).

## Kernel integration

Embedded + spawned via signed `spawn_verified`
(`src/userspace/capsule_image_codec/`), cfg-gated by
`nonos-capsule-image-codec` in `src/userspace/init/spawn_plan/desktop_services.rs`,
and in the `microkernel-desktop-offline` feature set that the desktop
builds on. Build with `make nonos-mk-image-codec`.
