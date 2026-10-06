# capsule_qrgen

`qrgen` opens a 420 by 470 window titled "qr generator" and draws the QR code for one fixed string,
`https://nonos.systems`. It is a demonstration that an unmodified crates.io crate (`qrcode` 0.14.1,
default features off) builds against the NONOS std layer and paints through the toolkit; there is no
input field, so it is not yet a tool an end user would reach for.

## Role

A std application: `Capsule.mk` sets `CAPSULE_BUILD_STD := std,panic_abort` and `src/main.rs` is
an ordinary `fn main` that calls `nonos_app_skeleton::run_loop`. Like the other skeleton apps it
waits on its service inbox for a focus control frame from `desktop_shell`
(`userland/app_skeleton/src/runner/idle.rs`) before opening the window. The `Cargo.toml` header
says it ships only through the on-disk package store.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x1819`, commented in `Capsule.mk` as
`CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate`:

- `0x0001` CoreExec: run.
- `0x0008` IPC: compositor, wm, input_router and toolkit.
- `0x0010` Memory: heap for the module matrix.
- `0x0800` GraphicsDisplayQuery and `0x1000` GraphicsSurfaceCreate: its window surface.

Endpoints: service `service:4924:app.qrgen`, reply `reply:4925:endpoint.app.qrgen.reply`. The
domain is `com.example`.

## Interface

It serves no IPC requests of its own. `src/qrgen/code.rs` encodes `PAYLOAD` once, when the app
is built, into a `Matrix` of dark and light modules. `src/qrgen/draw.rs` picks the largest integer
module scale that fits the window with a four module quiet zone (`src/qrgen/theme.rs`) and centres
it. `src/qrgen/paint.rs` draws the title, the payload text, the code, and "encode failed" if
encoding returned an error. `src/qrgen/event.rs` handles only Esc, which closes the window.

## State and privacy

The only state is the encoded matrix in memory (`src/qrgen/app.rs`). It reads no files, writes
nothing, and holds no FileSystem, Network or Debug bit.

## Build and test

- `make nonos-mk-qrgen` builds the ELF (the include is in `mk/20-build.mk`);
  `make nonos-mk-qrgen-sign` produces the certificate, manifest and trailer.

No proof crate or host test covers this capsule.

## Not done yet

- The payload is the constant `PAYLOAD` in `src/qrgen/code.rs`; there is no way to enter text.
- The `Cargo.toml` header says it ships through the on-disk package store, but `mk/40-run.mk` does
  not pack `qrgen` into the store image, and no launch path for it was found outside this directory
  and the signed artifacts under `nonos-data/trust/capsules/`.

Handbook: [app model](../../docs/handbook/desktop/app-model.md),
[capsule catalogue](../../docs/handbook/apps/capsule-catalog.md).
