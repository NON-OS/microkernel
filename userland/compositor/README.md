# compositor

## Role

`compositor` is the userland window compositor. It owns the scene
(one layer per client process), damage tracking, the frame loop, the
software blitter and the IPC client into `driver.virtio_gpu0`. Every
pixel a window shows goes through this capsule. It presents either
through the virtio-gpu driver or, on any other UEFI machine, through the
kernel's framebuffer blit (`MkSurfacePresent`).

```text
windowing apps (terminal, calculator, desktop_shell, ...)
    |
    | scene_submit / damage_commit / cursor_update / scene_remove
    v
compositor (this capsule) <--- focus_set (raise) from wm only
    |
    | virtio path: gfx_client (transfer_to_host / set_scanout / flush)
    | GOP path:    MkSurfacePresent on its own registered surface
    v
driver.virtio_gpu0 or the firmware framebuffer --> display
```

The handbook page is [Compositor](../../docs/handbook/desktop/compositor.md).

## Microkernel contract

- `MkIpcRecv` on port `4310` reads scene, damage, focus and cursor
  requests from windowing capsules.
- `MkSurfaceAttach` maps each window's surface into this capsule's
  address space; the attach cache keeps one mapping per handle.
- On the virtio path `gfx_client` asks `driver.virtio_gpu0` for its
  primary surface, attaches it, and drives `transfer_to_host`,
  `set_scanout` and `resource_flush` over IPC.
- On the GOP path the compositor maps a buffer, registers and attaches it
  with `MkSurfaceRegister`, and presents with `MkSurfacePresent`. While in
  GOP mode it probes for the virtio driver every `VIRTIO_PROBE_FRAMES`
  frames and moves over when it answers (`setup/upgrade.rs`).
- `MkServiceLookup` finds `wm` on every `OP_FOCUS_SET`, and the GPU
  driver at setup.
- The frame loop does not call `MkDisplayVsyncWait`: the first blocking
  receive of each drain, up to 16 ms, paces it. `frame_pacer/vsync.rs`
  wraps the call but nothing calls it.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_SCENE_SUBMIT` | 0x0002 | submit or update the sender's layer: handle, x, y, width, height, z |
| `OP_DAMAGE_COMMIT` | 0x0003 | mark a rectangle dirty |
| `OP_FOCUS_SET` | 0x0004 | raise a pid's layer to the top of its band; the window manager only, anyone else gets E_PERM |
| `OP_INPUT_SUBSCRIBE` | 0x0005 | sets the compositor's focus record to the sender; restacks nothing |
| `OP_CURSOR_UPDATE` | 0x0006 | move or hide the software cursor: x, y, visible |
| `OP_SCENE_REMOVE` | 0x0007 | remove the sender's layer |
| `OP_DISPLAY_INFO` | 0x0008 | canvas width, height, stride and format |

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
| 0x0008 | IPC | recv on port 4310 + send to driver.virtio_gpu0 |
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

## Runtime lifecycle

1. `_start` initialises the heap; `wait_for_setup` asks for the virtio
   GPU first and, after `VIRTIO_ATTEMPTS_BEFORE_GOP` failed rounds, also
   tries the firmware framebuffer, taking whichever answers.
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
- `gfx_client` calls use a 100 ms reply timeout, so a lost reply costs a
  late frame; `scanout_error_reported` notes a failed scanout once and
  the loop continues.
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
| Frame loop (paint, cursor, present) | `frame_pacer/{tick,composite,cursor}.rs` |
| gfx_client (driver.virtio_gpu0 IPC) | `gfx_client/{get_primary,transfer,set_scanout,flush}.rs`, `gfx_client/wire/*.rs` |
| Software blitter and 2x upscale | `sw_blitter/*.rs` |
| Wire protocol (header/ops/errno/limits) | `protocol/*.rs` |
| Setup (virtio, GOP, canvas, upgrade) | `setup/*.rs`, `wait_for_setup.rs` |

## Wire format

20-byte header followed by a typed payload, all fields little-endian:
magic `0x4E43_4D50` ("NCMP"), version 1, op at offset 6, flags at 8,
request id at 12, payload length at 16. `parse` requires the length to
match the message exactly. Payloads are at most 256 bytes.

## State ownership

`Context` (`state/context.rs`) owns: gfx_port, resource_id, width,
height, stride, backing_len, backing_va, gop_mode, surface_handle,
first_scanout_done, scanout_error_reported, next_request_id, scene,
damage, focus, cursor, attach, scale and screen. There is no shared
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
- No hardware cursor; the cursor is drawn into each damaged rectangle.
- No compositing effects beyond alpha-blended copy.
- One display, number 0.
- A client can still submit another process's surface handle as its own
  layer; the compositor does not check who shared it.

## Verification

- `userland/compositor_proofs/` runs the real blitter, damage, scene,
  raise rule and protocol parser on the host.
- `userland/desktop_proofs/` drives one press through the compositor
  scene, the window manager and the input router together.
- `make nonos-mk-host-trust-verify` verifies
  the baked `compositor.manifest.bin` against the trust anchor.
