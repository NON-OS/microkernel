# capsule_gui_demo

`gui_demo` is the smallest windowed capsule in the tree: it opens a 380 by 220 window titled
"GUI Demo", counts pointer clicks and closes on Esc. It exists for people bringing up or checking
the desktop path from the on-disk package store, not for end users; the `Cargo.toml` header says it
is shipped only through the store and never embedded in the kernel image.

## Role

A `no_std` application on `nonos_app_skeleton` (`src/main.rs` calls `nonos_app_skeleton::run`).
The skeleton finds the compositor, wm, input_router and toolkit services, then waits on the service
inbox until `desktop_shell` sends a focus control frame (`userland/app_skeleton/src/runner/idle.rs`)
before it builds the app and opens the window. `mk/40-run.mk` packs the ELF, certificate, manifest
and trailer into the QEMU block store under `/capsules/gui_demo.*` as one of the demo capsules the
desktop offers from the store.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x1819` (no decoding comment in `Capsule.mk`; decoded from
`userland/nonos_cap/src/bits.rs`):

- `0x0001` CoreExec: run.
- `0x0008` IPC: talk to the compositor, wm, input_router and toolkit.
- `0x0010` Memory: heap.
- `0x0800` GraphicsDisplayQuery and `0x1000` GraphicsSurfaceCreate: create and size its window
  surface.

Endpoints: service `service:4916:app.gui_demo`, reply `reply:4917:endpoint.app.gui_demo.reply`.
The domain is `com.example` and the namespace `com.example.gui_demo`.

## Interface

It serves no IPC requests of its own. The window manifest is in `src/gui_demo/manifest.rs` (window
id `0x4744454D`, key-down and button-down input). `src/gui_demo/event.rs` adds one to the counter on
every button press (saturating) and returns `Close` on Esc. `src/gui_demo/paint.rs` draws the title,
a panel with the count rendered by `src/gui_demo/decimal.rs`, and the hint line.

## State and privacy

The only state is a `u32` click counter in `src/gui_demo/app.rs`, held in memory and dropped with
the process. It reads no files, holds no FileSystem, Network or Debug bit, and never sees key
material.

## Build and test

- `make nonos-mk-gui_demo` builds the ELF; `make nonos-mk-gui_demo-sign` produces the certificate,
  manifest and trailer.
- The store image that carries it is `$(QEMU_BLK_STORE_STAMP)` in `mk/40-run.mk`, a prerequisite
  of `nonos-mk-run-from-config`.

No proof crate or host test covers this capsule.

## Not done yet

Nothing recorded.

The app model it is built on is described in
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).
