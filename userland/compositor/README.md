# compositor

## Role

`compositor` is the userland window compositor. It owns the scene
(one layer per client process), damage tracking, the frame loop, the
software blitter and the client of the display wire (NVGP v1,
PLAN-coverage section 12). Every pixel a window shows goes through this
capsule. It presents through the first display driver in the kernel's
list (`src/services/registry/displays.rs`) that answers, or, on any
other UEFI machine, through the kernel's framebuffer blit
(`MkSurfacePresent`).

```text
windowing apps (terminal, calculator, desktop_shell, ...)
    |
    | scene_submit / damage_commit / cursor_update / scene_remove
    v
compositor (this capsule) <--- focus_set (raise) from wm only
    |
    | wire path:   display/ (CAPS, SURFACE, STATE, FLUSH, RETIRE)
    | legacy path: gfx_client (transfer_to_host / set_scanout / flush)
    | GOP path:    MkSurfacePresent on its own registered surface
    v
a display driver (driver.bga0, ...), driver.virtio_gpu0, or the
firmware framebuffer --> display
```

The handbook page is [Compositor](../../docs/handbook/desktop/compositor.md).

## Microkernel contract

- `MkIpcRecv` on port `4310` reads scene, damage, focus and cursor
  requests from windowing capsules.
- `MkSurfaceAttach` maps each window's surface into this capsule's
  address space; the attach cache keeps one mapping per handle.
- A driver found is asked CAPS first (`setup/prime_wire.rs`). On the
  wire path it then asks SURFACE, which makes it the primary's owner (a
  restarted compositor gets its surface back), attaches the primary,
  sends each frame's damaged rectangles in one FLUSH (up to 15), polls STATE every 500 ms with
  the driver's pid looked up again, and on a new primary attaches it,
  releases the old handle and sends RETIRE (`setup/{poll,adopt}.rs`).
  Every output past 0 that CAPS counts (up to the wire server's four)
  mirrors the screen: its primary attached, each frame's rectangles drawn
  onto it fitted and centred (`sw_blitter/resample.rs`) and sent in one
  FLUSH per output (`frame_pacer/mirror.rs`); the poll reads each one's
  STATE and CAPS' output count, and a monitor plugged in or pulled out
  primes the mirrored outputs again (`setup/{heads,poll_heads}.rs`). The
  shell may extend instead (OUTPUT_LAYOUT): one canvas across every output
  side by side, output 0's canvas at the left and each further output a
  slice its screen holds at the same scale (`display/layout_rule.rs`,
  `setup/layout.rs`); each composed rectangle's part in a slice is drawn on
  that output and flushed once a frame (`frame_pacer/extend.rs`); outputs
  coming or going lay the canvas out again; the pointer is composited.
  Each output has its own scale in quarters (OUTPUT_SCALE; default the
  panel rule, whole steps as the kernel's boot console): whole steps go
  through `upscale`, 1.25 and 1.5 through `sw_blitter/stretch.rs` (each
  screen pixel the canvas pixel its centre falls in), and the pointer and
  damage are mapped through the same arithmetic (`display/scale_rule.rs`).
  The host is followed (G 6, `display/follow.rs`'s FOLLOW_HOST, Decision 15
  as the lead recommends it): on a steady driver with MODE_SET the poll asks
  the preferred mode at start, then any size the host has wanted for 250 ms
  (STATE's hint or a DISPLAY_HINT), put through `fit`; one MODE_SET, the new
  primary adopted at once and the canvas refitted; a size asked once is not
  asked again (`setup/resize.rs`, `display/resize_rule.rs`).
  A driver whose CAPS has HW_CURSOR gets the pointer on its cursor plane:
  the arrow is drawn into each primary's cursor slot (scaled with the
  screen, transparent around it) and announced with NEW_SHAPE, then each
  frame the pointer moved sends one CURSOR per output, its position mapped
  onto the screen and into each mirrored output's fitted box; the arrow is
  then neither composited nor damaged around. A refused CURSOR hides the
  plane everywhere and composites the arrow again until the primary is
  replaced (`frame_pacer/{hw_cursor,hw_cursor_back,cursor_shape}.rs`,
  `display/cursor_rule.rs`).
- A driver that answers CAPS with -38 is driven on the legacy path:
  `gfx_client` asks for its primary surface, attaches it, and drives
  `transfer_to_host`, `set_scanout` and `resource_flush` over IPC.
- On the GOP path the compositor maps a buffer, registers and attaches it
  with `MkSurfaceRegister`, and presents with `MkSurfacePresent`. While in
  GOP mode it asks one name of the display list per tick, 500 ms apart and
  backing off to 30 s, and moves over when one answers
  (`setup/upgrade.rs`, `display/backoff.rs`).
- `MkServiceLookup` finds `wm` on every `OP_FOCUS_SET`, and the GPU
  driver at setup.
- The frame loop does not call `MkDisplayVsyncWait`: the first blocking
  receive of each drain, up to 16 ms, paces it. `frame_pacer/vsync.rs`
  wraps the call but nothing calls it.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_SCENE_SUBMIT` | 0x0002 | submit or update the sender's layer: handle, x, y, width, height, z; a z at or above `TRUSTED_Z` (0x4000_0000), the trusted band where the lock screen and permission prompts are drawn, only from a trusted source (the desktop shell), E_PERM otherwise (`state/trusted_rule.rs`) |
| `OP_DAMAGE_COMMIT` | 0x0003 | mark a rectangle dirty |
| `OP_FOCUS_SET` | 0x0004 | raise a pid's layer to the top of its band; the window manager only, anyone else gets E_PERM |
| `OP_INPUT_SUBSCRIBE` | 0x0005 | sets the compositor's focus record to the sender; restacks nothing |
| `OP_CURSOR_UPDATE` | 0x0006 | move or hide the software cursor: x, y, visible |
| `OP_SCENE_REMOVE` | 0x0007 | remove the sender's layer |
| `OP_DISPLAY_INFO` | 0x0008 | canvas width, height, stride and format |
| `OP_DISPLAY_HINT` | 0x0009 | a host size hint (output, width, height, source) -> flags; only from a service in `HINT_SOURCES` (empty until the host-services decision), others get E_PERM; for output 0 of a wire driver held and followed like a STATE hint and answered 1 (resizable), else 0 |
| `OP_INDICATOR` | 0x000A | a capturing driver's camera (kind 1) or microphone (kind 2) on (1) or off (0), 8 bytes; only from a holder of that kind's capture capability bit (CameraCapture 1<<37, MicrophoneCapture 1<<38; no capsule may hold either until Decision 9), others get E_PERM, shown in the reserved band and presented before the answer (E_IO and no change if the present fails), E_BUSY past eight holders |
| `OP_OUTPUT_LAYOUT` | 0x000B | the shell's choice for several outputs: mode u32, 0 mirror or 1 extend; only from desktop_shell (E_PERM otherwise); kept while one output is present; answers u32 1 when extended now, else 0 |
| `OP_SCREENSHOT` | 0x000C | the scene composed again into the caller's own shared ARGB surface, without the trusted band, the cursor, the tint or the band: handle u64; only from a holder of the screen capture bit (ScreenCapture 1<<39; no capsule may hold it until Decision 9), others get E_PERM; the band's blue mark shows for 3 s, presented before the copy; answers width and height copied, u32 each |
| `OP_OUTPUT_SCALE` | 0x000D | the shell's scale for one output: output u32, quarters u32 (0 the panel's own rule, else 4, 5, 6, 8, 12 or 16: 1, 1.25, 1.5, 2, 3, 4 times); only from desktop_shell; the canvas is laid out again; answers output 0's scale in quarters |
| `OP_NIGHT_LIGHT` | 0x000E | the shell's colour temperature: kelvin u32 (0 or 6500 and above true colour, 1000 the warmest); only from desktop_shell; every composed pixel looked up through three ramps (`display/night_rule.rs`, `frame_pacer/night_tint.rs`), the indicator band kept true; the shell keeps the schedule |
| `OP_ACCESSIBILITY_GET` | 0x0010 | one output's settings now: output u32 in, filter, contrast, magnify and follow u32 each out; only from desktop_shell, which asks for Settings (`server/handlers/accessibility_get.rs`) |
| `OP_ACCESSIBILITY` | 0x000F | one output's accessibility, as Settings chose it: output, filter (0 none, 1 greyscale, 2 protan, 3 deutan, 4 tritan), contrast (0 or 1), magnification in quarters (4, 6, 8, 12, 16, 24 or 32; 4 is off), follow (0 the pointer, 1 the focused window), u32 each; only from desktop_shell, E_INVAL past four outputs or a step it does not have. Filters work on linear light with matrices derived in `display/cvd_rule.rs`, `cvd_filter.rs` and `colour_rule.rs`, then night light, then contrast (`frame_pacer/look.rs`); the magnifier draws each output through a lens on its own canvas (`display/magnifier_rule.rs`, `sw_blitter/magnify.rs`). Mirrored outputs show output 0's |

The window manager owns the stacking order, and `OP_FOCUS_SET` is how it keeps the compositor's
order the same. The compositor looks the window manager's pid up in the service registry on every
such request (`state/raise_rule.rs`), so no other client can put its window over another's, and a
restarted window manager is recognised at once.

Inside a `z` band, layers draw in raise order (`state/scene/raise.rs`,
`state/scene/snapshot.rs`): a layer submitted for the first time goes on
top, a resubmit keeps its place, and only a raise moves it. Applications
submit at `z` 2 and the desktop shell's overlay at 1.

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x7818` and `CAPSULE_OPTIONAL_CAPS := 0x100`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0008 | IPC | recv on port 4310 + send to the display drivers |
| 0x0010 | Memory | scene, damage and attach cache; the GOP and canvas buffers |
| 0x0800 | GraphicsDisplayQuery | read the framebuffer size on the GOP path |
| 0x1000 | GraphicsSurfaceCreate | register the GOP present surface |
| 0x2000 | GraphicsSurfaceMap | map each window's surface and the GPU's primary surface |
| 0x4000 | GraphicsPresent | `MkSurfacePresent` on the GOP path |

`Debug` is optional: only a `capsule-serial-debug` build grants it, and
the compositor's own code makes no debug call. No `CoreExec`, `Driver`,
`Mmio`, `Irq`, `Dma`, `Pio`, `Network`, `Crypto`, `FileSystem`,
`Hardware`, `Admin` or `RegisterService` capability is requested.

## Privacy posture

| Invariant | How `compositor` honors it |
|---|---|
| NO LOGS | No debug output; the spawn `debug_tag` is the empty string. |
| NO TRACES | Scene, damage, focus and cursor live only in process memory. No frame is written anywhere. |
| EPHEMERAL | Zero files. The attach cache (`state/attach.rs`) holds surface mappings for the life of each handle. |
| NOT LINUX | NONOS Mk syscall ABI. The wire format is the NCMP header, not Wayland. |
| PRIVACY MICROKERNEL | Six required bits, all graphics or IPC. No Network, FileSystem or Crypto path. |
| TRUSTED PATH | The lock screen and permission prompts are drawn in the trusted band: no client layer can be placed in it or raised into it, no screenshot holds it, and the input router gives its holder every key and click while it is up (`server/trusted_notice.rs`). |

## Runtime lifecycle

1. `_start` initialises the heap; `wait_for_setup` asks for the first
   announced display driver and, after 500 ms with none, also tries the
   firmware framebuffer, taking whichever answers.
2. `fit_canvas` gives clients a half-size canvas on panels of 2560 by
   1440 or more (`setup/canvas_scale.rs`); each damaged rectangle is
   doubled onto the screen.
3. Only then does it register the `compositor` service.
4. The loop alternates between `drain_ipc` and `tick`. `tick` paints each
   damaged rectangle: background, every layer bottom to top through the
   attach cache, then the cursor, and presents it.
5. Every 240 frames the whole screen is marked damaged so a region an
   under-reporting client left stale heals.

## Failure model

- No display path yet: `wait_for_setup` keeps retrying; the service is
  not registered until one answers.
- `mk_surface_attach` on a layer's handle fails for 60 frames in a row:
  the layer is dropped and its rectangle repainted.
- `gfx_client` calls and FLUSH use a 100 ms reply timeout, so a lost
  reply costs a late frame; `scanout_error_reported` notes a failed
  present once and the loop continues. A refused FLUSH leaves its
  rectangle damaged for the next frame; ESTALE also has the next pass ask
  SURFACE again.
- A malformed request gets a typed errno (`E_BAD_MAGIC`, `E_BAD_VERSION`,
  `E_BAD_LEN`, `E_BAD_OP`, `E_INVAL`, `E_PERM`).

## Current implemented surface

| Concern | File |
|---|---|
| Entry + IPC drain loop | `server/runner/{entry,drain,dispatch}.rs` |
| Per-op handlers | `server/handlers/{health,scene_submit,scene_remove,damage_commit,focus_set,cursor_update,input_subscribe,display_info}.rs` |
| Reply builder | `server/respond.rs` |
| Scene table, raise order, reaping | `state/scene/*.rs`, `state/scene_raise.rs`, `state/scene_submit.rs`, `state/scene_remove.rs` |
| Who may raise | `state/raise_rule.rs` |
| Damage accumulator | `state/damage.rs` |
| Focus record | `state/focus.rs` |
| Cursor tracker | `state/cursor.rs` |
| Attach cache | `state/attach.rs` |
| Context (owned by runner) | `state/context.rs` |
| Frame loop (paint, cursor, present) | `frame_pacer/{tick,to_screen,composite,cursor,drain_batch,hw_cursor}.rs` |
| gfx_client (driver.virtio_gpu0 IPC) | `gfx_client/{get_primary,transfer,set_scanout,flush}.rs`, `gfx_client/wire/*.rs` |
| Software blitter and 2x upscale | `sw_blitter/*.rs` |
| Wire protocol (header/ops/errno/limits) | `protocol/*.rs` |
| Display wire client (calls, back-off, poll and hint rules) | `display/*.rs`, `frame_pacer/present_wire.rs`, `server/runner/upkeep.rs`, `server/handlers/display_hint.rs` |
| Setup (wire, legacy, GOP, canvas, upgrade, poll) | `setup/*.rs`, `wait_for_setup.rs` |

## Wire format

20-byte header followed by a typed payload, all fields little-endian:
magic `0x4E43_4D50` ("NCMP"), version 1, op at offset 6, flags at 8,
request id at 12, payload length at 16. `parse` requires the length to
match the message exactly. Payloads are at most 256 bytes.

## State ownership

`Context` (`state/context.rs`) owns: gfx_port, resource_id, width,
height, stride, backing_len, backing_va, gop_mode, surface_handle,
first_scanout_done, scanout_error_reported, next_request_id, scene,
damage, focus, cursor, attach, scale, screen and wire (the driver on
the display wire: its name, pid, primary serial and CAPS' panel size).
The NVGP request id is the one shared static (`display/call.rs`). There is no other shared
static state. Per-window state is keyed by the caller's pid.

## Operating rules

- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Small files, one non-trivial function per file where it fits;
  `mod.rs` carries re-exports.

## Release target

x86_64-nonos-user. Cross-compiled with the kernel-pinned nightly
toolchain.

## Release evidence

`cargo check --features microkernel-core,nonos-production,nonos-capsule-compositor`
must compile clean. The capsule's own
`cd userland/compositor && cargo build --release --target ../x86_64-nonos-user.json`
must produce a signed ELF whose SHA matches the embedded manifest
`nonos-data/trust/capsules/compositor.manifest.bin`.

## Release checklist

- [x] `Capsule.mk` with `CAPSULE_REQUIRED_CAPS = 0x7818`
- [x] Capability mask audited (6 required bits, Debug optional)
- [x] Kernel mirror at `src/userspace/capsule_compositor/`
- [x] Cert + manifest baked into `nonos-data/trust/capsules/`
- [x] Spawn wired through `src/userspace/init/spawn_plan/`
- [x] Virtio present path with a GOP fallback and live upgrade
- [x] Only the window manager raises (`OP_FOCUS_SET` answers E_PERM to others)
- [x] Host proofs in `userland/compositor_proofs/`

## Explicit non-goals today

- No GPU 3D acceleration. The compositor is a 2D blitter.
- A hardware cursor only on a wire driver with HW_CURSOR; otherwise the
  cursor is drawn into each damaged rectangle.
- No compositing effects beyond alpha-blended copy.
- One display, number 0.
- A client can still submit another process's surface handle as its own
  layer; the compositor does not check who shared it, and the kernel's
  attach names no receiver (reported to the lead; the fix is kernel-side,
  `mk_surface_share_to`).
- No op reads back pixels, a surface or a handle to a client; the screen's
  pixels reach only a screen capture holder's own surface, with the band's
  notice shown first (`privacy_tests`, `screenshot_tests`).
- The capture indicator's band (top right, 44x16 canvas pixels) is drawn
  after every layer in every frame that touches it, so no surface covers it
  (`frame_pacer/indicator_band.rs`).
- INPUT_SUBSCRIBE no longer makes the sender the focused process: input is
  the input router's to deliver, to the window the window manager focused.

## Verification

- `userland/compositor_proofs/` runs the real blitter, damage, scene,
  raise rule and protocol parser on the host.
- `userland/desktop_proofs/` drives one press through the compositor
  scene, the window manager and the input router together.
- `make nonos-mk-host-trust-verify` verifies
  the baked `compositor.manifest.bin` against the trust anchor.
