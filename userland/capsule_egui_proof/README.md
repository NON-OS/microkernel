# capsule_egui_proof

`capsule_egui_proof` is a demonstration capsule that runs the unmodified `egui` 0.35 crate in
a user capsule, lays out a small UI each frame, tessellates it, and paints the resulting
counts (shapes, meshes, vertices, indices, texture atlas size) into a window. Its
`Cargo.toml` description is "unmodified egui 0.35 laying out and tessellating at CPL=3". It is
built against std (`CAPSULE_BUILD_STD := std,panic_abort` in `Capsule.mk`).

## Role

`src/main.rs` hands `EguiProof::new` to `nonos_app_skeleton::run_loop`. Each paint,
`src/egui_proof/frame.rs` runs one `egui::Context::run_ui` pass (a heading, a separator, a
click counter label and a button) over a 460 by 300 screen rect, tessellates the output, and
counts the mesh primitives. `src/egui_proof/paint.rs` draws those counts with the skeleton's
own text renderer and shows "EGUI PROOF PASS" when vertices and indices are both nonzero,
otherwise "EGUI PROOF FAIL".

## Capabilities

From `Capsule.mk`:

```make
CAPSULE_REQUIRED_CAPS    := 0x1819
```

There is no comment on the value. Decoded against `src/capabilities/types/defs.rs` it is
CoreExec, IPC, Memory, GraphicsDisplayQuery and GraphicsSurfaceCreate. No hardware, network,
filesystem or debug bits are set.

## Interface

- Service endpoint `service:4918:app.egui_proof`, reply endpoint
  `reply:4919:endpoint.app.egui_proof.reply`, handle `egui_proof`.
- Window manifest (`src/egui_proof/manifest.rs`): title "egui proof", 460 by 300 at
  (380, 240), input mask key down and button down.
- Any input event increments the click counter and requests a repaint
  (`src/egui_proof/app.rs`).

## State and privacy

State is two counters (clicks, frames) and the `egui::Context`, all in capsule memory. The
capsule reads no files and opens no connections.

## Build and test

- Build: `make nonos-mk-egui_proof` (target pattern from `nonos-mk/capsule.mk`); the release
  profile in `Cargo.toml` uses `opt-level = "z"`, LTO and `panic = "abort"`.
- There is no host test crate for this capsule. The on-screen PASS line is the only check,
  and it tests only that tessellation produced a nonempty mesh.

## Not done yet

- egui's meshes are counted, not rasterised: what appears on screen is the skeleton's text,
  not egui's output.
- Input is not forwarded to egui. `frame.rs` builds `RawInput` with only `screen_rect`, so the
  button cannot be clicked through egui; the counter is driven by `on_event` instead.
- `Capsule.mk` sets no `CAPSULE_FEATURE` or `CAPSULE_KERNEL_MIRROR`, which
  `capsule_socks5` and `capsule_install` both set.

The app model it is built on is described in
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).
