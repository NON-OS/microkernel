# toolkit

`nonos_toolkit` is the drawing library every NONOS desktop app links, and
the same crate builds a small `toolkit` service capsule. All drawing
happens in the app's own address space: the library paints into the
app's buffer, and `app_skeleton` re-exports its `paint` module as
`paint`. How an app is put together is in
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).

## The library

`src/lib.rs` is `no_std` and exposes these modules:

| Module | What it holds |
|---|---|
| `paint` | fills, lines, rounded rects, gradients, shadows, blits, blending, bitmap and TrueType text |
| `font` | the bitmap atlas and TrueType rendering through `ab_glyph` (`font/ttf`) |
| `decorations` | the window frame: titlebar, the close, minimize and maximize buttons, `hit_test`, the titlebar accessory rect |
| `widgets` | the shared widgets every app draws with, and the text helpers |
| `design` | colour, border, shadow, spacing and typography tokens |
| `theme` | colour schemes, contrast checks, derived palettes, the theme store |
| `image` | decoders for BMP, GIF, JPEG and PNG, raw LZ4 pixels, scaling |
| `icons` | the icon table and its painter |
| `qr` | a QR encoder (ECC, format, mask, placement, render) |
| `animation` | easing, timing and transitions |
| `protocol`, `server` | the service capsule below |

`image` is what `image_codec` decodes with, and `capsule_wallpaper`
decodes its JPEGs with the same code.

## The service capsule

`src/main.rs` builds the `toolkit` capsule.

| Field | Value |
|---|---|
| Service | `service:4610:toolkit` |
| Reply | `reply:4611:endpoint.toolkit.reply` |
| `CAPSULE_REQUIRED_CAPS` | 0x19: CoreExec, IPC, Memory |
| Kernel mirror | `src/userspace/capsule_toolkit`, spawned by `spawn_toolkit` in `src/userspace/init/spawn_plan/desktop_services.rs` |

Requests use a 16-byte header with magic `0x4E4F_544B` ("NOTK"). The ops:

| Op | Value | Reply |
|---|---|---|
| `TOOLKIT_OP_HEALTHCHECK` | 0x0000 | status |
| `TOOLKIT_OP_ANIMATION_TICK` | 0x0002 | the animation step |

0x0001 (theme apply), 0x0003 (component render) and 0x0004 (theme get) are
retired: nothing sent them, so they were removed, and a request naming one
is refused with `E_BAD_OP`. They are never reused.

Status codes are `STATUS_OK`, `E_BAD_OP`, `E_INVAL` and `E_SHORT`.
`E_SURFACE` stays defined but nothing returns it since component render was
retired. A frame whose header `decode` refuses still gets a header-only
reply from `refusal`: `E_SHORT` when it is shorter than a header,
`E_INVAL` otherwise, so the caller does not wait out its timeout.

## What it does not do

- It draws nothing for a client and holds no theme of its own; apps draw
  with the library and read the theme from their own process.

## Tests

`userland/theme_proofs` runs the theme engine, contrast rules, text scale,
icons and every shared widget on the host. `userland/image_paint_proofs`
runs the image decoders and paint primitives against fixtures, and
`userland/image_codec_proofs` fuzzes the decoders through `image_codec`.
`tests/host` holds host programs for the frame geometry, the icon table,
the mask format and resampling; the crate has no `[[test]]` entry for them,
so `cargo test` does not run them.
