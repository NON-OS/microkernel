# capsule_wallpaper

## Role

`capsule_wallpaper` paints the desktop background. it registers one
full-screen surface, submits it to the compositor at `z` 0, below the desktop
shell's overlay and every application window, and keeps it painted with the
wallpaper the policy store names. the chosen images come from the wallpaper
catalog over IPC; a built-in picture is linked in so the background is up
before any fetch returns. the handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
policy (Field::Wallpaper) --> wallpaper --> wallpaper catalog (JPEG bytes)
                                  |
                                  | decode, stretch, paint into the backing
                                  v
                    own surface at z 0 --> compositor (OP_SCENE_SUBMIT)
```

## Microkernel contract

- `MkServiceLookup` (`mk_service_lookup`) finds the `compositor`, the `policy`
  store, the catalog and the trusted setters.
- `MkMmap` and `MkMunmap` (`mk_mmap`, `mk_munmap`) allocate the backing, and
  `MkSurfaceRegister` and `MkSurfaceShare` (`mk_surface_register`,
  `mk_surface_share`) turn it into a surface the compositor can map, sized from
  the compositor's `OP_DISPLAY_INFO`.
- `MkIpcCall` (`mk_ipc_call_timeout`) submits the scene and commits damage to
  the compositor, asks `policy` for `Wallpaper` and fetches catalog images in
  chunks.
- `MkIpcRecvFrom` and `MkIpcReply` (`mk_ipc_recv_from`, `mk_ipc_reply`) serve
  `service:4340:wallpaper`. `MkDisplayVsyncWait` (`mk_display_vsync_wait`) is
  the idle clock and the fade clock; `MkYield` gives up the slice when there is
  no frame to draw.
- the kernel mirror is `src/userspace/capsule_wallpaper`, under the feature
  `nonos-capsule-wallpaper`.

## Interface contract

requests use a 20-byte header with magic `0x4E57_4C50` ("NWLP"), version 1:

| op | value | purpose |
|---|---|---|
| `OP_HEALTHCHECK` | `0x0001` | liveness ping, empty body |
| `OP_SET_WALLPAPER` | `0x0002` | paint a colour or an image carried in the request (PNG, BMP, JPEG or raw LZ4) |
| `OP_GET_WALLPAPER` | `0x0003` | the current colour, placement policy, size and alpha |
| `OP_SET_POLICY` | `0x0004` | the placement: `Fill`, `Fit`, `Stretch`, `Center` or `Tile` |
| `OP_FADE` | `0x0005` | fade to a target alpha over a duration |

`OP_SET_WALLPAPER` is accepted only from the pids behind `desktop_shell` and
`policy`, looked up with `MkServiceLookup` on every request; any other sender
gets `E_ACCES` (`-13`). a frame whose header is refused is answered with
`E_BAD_MAGIC` (`-71`), `E_BAD_VERSION` (`-93`) or `E_BAD_LEN` (`-90`); an
unknown op with an empty body gets `E_BAD_OP` (`-38`).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x1818`: IPC (`0x08`), Memory (`0x10`),
GraphicsDisplayQuery (`0x800`) and GraphicsSurfaceCreate (`0x1000`).
GraphicsDisplayQuery is the bit `MkDisplayVsyncWait` needs; GraphicsSurfaceCreate
covers the mmap, register and share of the backing. there is no FileSystem
(images come from the catalog over IPC), and no Debug, network, driver or
admin authority. the kernel installs the mask from the verified manifest and
the capsule cannot widen it.

## Privacy and persistence

the capsule reads no user files and stores nothing. the chosen wallpaper is the
policy store's `Wallpaper` field, which is kept across reboots only as one of
the answers first-boot setup keeps on a persistent install. the capsule itself
holds only the current surface and the decoded image in its heap.

## Runtime lifecycle

1. `_start` sets up the heap, then `wait_for_setup` retries `setup/prime` until
   the compositor answers and the surface is registered and submitted
   (`src/setup/prime/run.rs`).
2. `prime` discovers the compositor port, healthchecks it, allocates the
   backing, fills it with ink (`0xFF0A_0B0D`), registers and shares the
   surface, paints the linked-in picture
   (`nonos-data/wallpapers/special-variant-9.jpg`) and submits the scene once.
   it does not fetch the chosen wallpaper here; that would block setup for
   seconds, so the built-in picture goes up first.
3. the server loop serves requests, keeps the surface registered and committed,
   and runs the fade. every `POLL_EVERY` (300) subscriber ticks it asks the
   policy store for `Wallpaper`; when the index changed it fetches that image
   from the catalog, decodes it with the toolkit's JPEG decoder into a buffer
   of at most 1920 by 1080 pixels, stretches it over the surface and commits
   damage.

## Failure model

- compositor not ready: setup is retried; the registered surface keeps naming
  the same backing so a busy compositor no longer sends setup back to the start
  and leaks a screen's worth of memory each round.
- a scene submit or damage commit that times out is treated as a busy
  compositor, not a lost request (`CALL_REPLY_TIMEOUT_MS` 16, `BOOT` 250): the
  kernel queued it when the call was sent.
- catalog or policy unreachable: the current background stays.
- an image that does not decode leaves the background as it was and the request
  gets `E_INVAL` (`-22`).

## Current implemented surface

the five ops above, the surface setup and keep-registered path
(`src/server/scene`), the policy subscriber that polls and fetches
(`src/subscriber`), the catalog and policy clients, the JPEG decode and blit
(`src/paint`), and the fade timeline. no second surface and no input handling.

## Wire format

magic `0x4E57_4C50`, version 1, a 20-byte little-endian header: magic `u32`,
version `u16`, op `u16`, flags `u16`, two pad bytes, `request_id` `u32`,
`payload_len` `u32`. a reply reuses the header, then a 4-byte `i32` status,
then the body.

`OP_SET_POLICY` is a 28-byte op on the wire: the 20-byte header followed by an
8-byte body of `policy` `u32` and a `u32` pad (`SET_POLICY_REQ_LEN` is 8). the
`policy` word is `Fill` 0, `Fit` 1, `Stretch` 2, `Center` 3 or `Tile` 4; any
other value is `E_INVAL`. `OP_SET_WALLPAPER` with an 8-byte body is a solid
colour (`argb` `u32`, `u32` pad); a longer body is decoded as an image.
`OP_FADE` takes 8 bytes (`target_alpha` `u32`, `duration_ms` `u32`).
`OP_GET_WALLPAPER` returns a 24-byte body (`argb`, `policy`, `width`,
`height`, `alpha`, pad). the scene submit the capsule sends the compositor
uses a separate protocol, magic `0x4E43_4D50` ("NCMP"), op `0x0002`, a 32-byte
body (`surface_handle` `u64`, `x`, `y`, `w`, `h`, `z`, pad), with `x` and `y`
0 and `z` `BOTTOM_Z` 0.

## State ownership

the capsule owns its backing (one mmap'd ARGB8888 buffer), the surface handle
the compositor maps, and the current `Context` (colour, alpha, placement
policy, the fade timeline and the subscriber plan). none of it is shared with
another process or persisted; the compositor maps the shared surface but the
backing's lifetime is the capsule's.

## Operating rules

- the background surface is submitted at `z` 0, strictly below the shell
  overlay and application windows, so it never covers them.
- setup never maps a new backing on a retry: the registered surface keeps the
  first one, and only a fresh start (nothing yet names the backing) maps its
  own.
- only `desktop_shell` and `policy` may set the wallpaper image or colour; the
  placement and fade ops carry no image and are not gated on the setter list.
- the subscriber fetches a new image only when the policy index changed, and
  decodes into a bounded 1920 by 1080 buffer, so a hostile catalog cannot make
  it allocate without limit.

## Release target

0.9.2.

## Release evidence

`userland/wallpaper_catalog_proofs` covers the catalog's request path on the
host; this capsule's own paint and subscriber paths are exercised at boot under
the desktop image rather than by a host proof. build and sign with
`make nonos-mk-wallpaper` and `make nonos-mk-wallpaper-sign`.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x1818`.
- [ ] `OP_SET_WALLPAPER` refuses a sender that is not `desktop_shell` or
      `policy`.
- [ ] the surface comes up at `z` 0 with the built-in picture before any
      catalog fetch.
- [ ] a retry of setup reuses the registered backing rather than mapping a new
      one.

## Explicit non-goals today

the capsule paints one full-screen background and nothing else: no second
surface, no input, no window management, and no writing of the chosen wallpaper
(that is the policy store's). it reads no files and keeps no history of past
wallpapers. it decodes JPEG, PNG, BMP and raw LZ4 only, into a fixed maximum
buffer, and does no scaling beyond the stretch to the surface.

## Verification

`src/protocol/header.rs`, `src/protocol/ops.rs` and `src/protocol/limits.rs`
fix the magic, ops and body lengths; `src/server/handlers/set_policy.rs` fixes
the 28-byte `OP_SET_POLICY` decode and the `Policy` values; `src/setup/prime/run.rs`
fixes the setup send path and the `z` 0 first submit; `src/subscriber/tick.rs`
fixes the 300-tick poll. every magic, op, length, errno and endpoint above is
set in those files, and `userland/wallpaper_catalog_proofs` exercises the
catalog side on the host.
