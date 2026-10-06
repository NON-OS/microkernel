# capsule_image_viewer

`image_viewer` is the desktop "Images" window: a thumbnail gallery of every PNG, JPEG, BMP and GIF
the vfs holds, and a single image view with zoom, pan, rotate, flip, fit modes, an info overlay and
a slideshow. It is for any desktop user, and the desktop shell can hand it a path to open. The handbook page
is [Audio and media](../../docs/handbook/audio-and-media.md).

## Role

A `no_std` application on `nonos_app_skeleton`. `src/main.rs` sizes the heap to 192 MiB and calls
`nonos_app_skeleton::run`. It decodes nothing itself: `src/viewer/decode.rs` shares the file bytes
with the `image_codec` service as a surface and copies back the ARGB surface it returns. The kernel
embeds it under `nonos-capsule-image-viewer` through the mirror at
`src/userspace/capsule_image_viewer/`.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x3859`, commented in `Capsule.mk` as `CoreExec|IPC|Memory|FileSystem|`
`GraphicsDisplayQuery|GraphicsSurfaceCreate|GraphicsSurfaceMap`, and `CAPSULE_OPTIONAL_CAPS := 0x100`:

- `0x0001` CoreExec and `0x0010` Memory: run, and hold decoded images.
- `0x0008` IPC: vfs, `image_codec`, `desktop_shell` and the window services.
- `0x0040` FileSystem: `list_paths` and `read_file` (`src/viewer/load.rs`, `src/viewer/gallery/`).
- `0x0100` Debug, optional: the `[IMG-VIEWER]` markers of the self-test build (`src/selftest.rs`);
  the normal build makes no `mk_debug` call. Only a `capsule-serial-debug` build grants it, so the smoketest
  kernel profile must compile that feature for the markers to appear.
- `0x0800`, `0x1000`, `0x2000` graphics: its window, plus registering, sharing and attaching the
  codec input and output surfaces.

Endpoints: service `service:4746:app.image_viewer`, reply
`reply:4747:endpoint.app.image_viewer.reply`.

## Interface

It serves no IPC requests; it is a client of three services:

- `image_codec`: magic `NIMG`, ops `0x0002` PNG, `0x0003` BMP, `0x0005` JPEG, `0x0006` GIF, chosen
  by file extension (`src/viewer/decode.rs`).
- `desktop_shell`: magic `NDSH`, op `0x0008` take-open-argument, asked by `src/viewer/arg.rs`;
  a path in the reply opens straight into the single view. While the window has nothing to show,
  in its first 3 s, it asks on every 150 ms tick, so a file it was launched for opens at once.
  Once the gallery or an image is up it asks every 850 ms, so a file opened while the viewer is
  up (the shell raises its window) shows within a second (`src/viewer/arg_cadence.rs`).
- vfs: `/` is scanned for images at the first gallery tick (`src/viewer/gallery/scan.rs`) and
  thumbnails are made one per tick, the tiles on screen first (`src/viewer/gallery/thumbs.rs`).
  Every image found is a tile; only the thumbnails of the rows on screen and one row either side
  are held, the rest are dropped and made again when scrolled back to (`layout::kept`), so a large
  store does not grow the heap. Arrow keys move the selection and the view follows it.

The window is 720 by 520 (`src/viewer/manifest.rs`). Keys in `src/viewer/app.rs`: Left and Right
step, `+ - 0` zoom, `f 1 w` fit, actual size and fill, `r` rotate, `h v` flip, `i` info, `?` help,
space slideshow, `[` faster and `]` slower by half a second, Esc or Backspace back to the gallery.
Wheel zooms, drag pans, a wide horizontal swipe or the side buttons (`src/viewer/nav.rs`) step;
the buttons show, and answer, whenever there is another image to step to, also when the one on
screen failed to decode. The help overlay lists the same keys (`src/viewer/caption.rs` `KEYMAP`),
and the info overlay gives the scale the image is drawn at, not the zoom factor over the fit.
Opened from the gallery, prev and next walk the gallery; opened by path, the images in its folder.
A slideshow needs more than one image and says so.

## State and privacy

Everything is in memory: the decoded image, the directory list, view and fit mode, slideshow
settings and gallery thumbnails (`src/viewer/state.rs`). Files over 16 MiB are refused, read with a limit one byte past that so a cut-off file is told
apart, and a decoded image over 20 million pixels is refused before it is copied into the heap
(`src/viewer/budget.rs`). The image on screen is dropped before the next is read. The viewer
claims its own pid from `mk_getpid` as owner with the vfs, so an on-demand instance is served
too; a gallery whose listing failed says why instead of "No images found", and a decoder that is
not running is said once over the gallery (`src/viewer/says.rs`). It writes
nothing to the vfs, so rotations and flips are never saved. It holds no Network bit.

## Build and test

- `make nonos-mk-image-viewer` and `make nonos-mk-image-viewer-sign` (the slug is `image-viewer`).
- `make nonos-mk-image-viewer-test` rebuilds it with `nonos-image-viewer-smoketest`, whose
  `src/selftest.rs` runs rotate, flip, nearest and bilinear scale and the gallery grid on baked
  pixels and prints `[IMG-VIEWER] PASS` or `FAIL`, then builds the matching kernel profile.

`userland/apps_proofs` includes `src/viewer/budget.rs` and `src/viewer/says.rs` and checks the
size limits and the gallery's empty and failure lines; it also includes `caption.rs`, `nav.rs`,
`viewport.rs` and `gallery/layout.rs` (`image_view_tests`): every image reachable, the kept
thumbnails bounded, the selection followed, the real scale, the interval and the help text.
`image_arg_tests` includes `arg_cadence.rs` and checks that an empty window asks on every tick
while it starts, then about once a second, and that a file handed to an open viewer is taken
within a second.

## Open

- `src/viewer/arg.rs` still polls `desktop_shell`, about once a second once the window shows
  something. The shell has no way to tell an app that a path is waiting for it; ending the poll
  needs that notification in `capsule_desktop_shell`.
