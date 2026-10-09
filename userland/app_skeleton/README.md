# app_skeleton

`nonos_app_skeleton` is the framework every NONOS desktop app is built on.
An app is a struct that implements the `App` trait and a `_start` that
calls `run`; the skeleton does the rest: it waits for the desktop services,
opens the window, subscribes to input, decorates and moves the window, and
runs the frame loop. It is a library, not a capsule. The full model is in
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).

## The App trait

`src/app/behavior.rs` defines it. Three methods are required: `manifest`
(title, window id, kind, size, position, the input kinds to subscribe to),
`on_event` (returns an `EventOutcome`: Idle, Repaint, Close, Minimize or
Maximize) and `paint`. The rest have defaults: `on_tick` and
`tick_interval_ms`, `busy`, `take_damage` for partial repaints, a titlebar
accessory widget (`titlebar_accessory_w`, `paint_accessory`,
`on_accessory_event`), and `close_requested`.

## What run does

1. `require_peers` waits for `compositor`, `wm`, `input_router` and
   `toolkit` to be registered; after 256 yields `run` exits with code 2.
2. `wait` parks until the desktop shell sends an `NCTL` `OP_FOCUS_SELF`
   frame; frames from any other sender are ignored. An app is resident
   from boot and has no window until asked.
3. `boot` fits the manifest to the display, maps an anonymous backing,
   registers and shares it as a surface, sends `OP_WINDOW_OPEN` to the
   window manager, submits the surface to the compositor at the placed
   rectangle and `z` 2 (`APP_LAYER_Z`), and subscribes to input.
4. `frame_loop` services input, ticks the app and repaints. A full paint
   is built in a private buffer and copied over the surface in one pass,
   so the compositor never reads a half-drawn frame.
5. On close it removes the scene, unsubscribes, releases the surface,
   unmaps the backing and closes the window. A base app goes back to
   `wait`; a window instance spawned with `--nonos-window-instance` exits.

## Window handling

Windows are decorated client side with the toolkit's `decorations`. A
press anywhere sends the window manager `window_raise` and `window_focus`
(`runner/click_focus.rs`). While a button is held, `PressGrab`
(`runner/press_part.rs`) keeps every pointer event on the part of the
window the press began on, the titlebar widget or the rest of the window,
until the release. A titlebar drag moves the window; the right and bottom
borders below the titlebar resize it (`runner/drag.rs`).

## Clients

`src/clients/` holds the IPC clients apps use directly:

| Module | Service | Calls |
|---|---|---|
| `compositor` | `compositor` | scene submit and remove, damage commit, display info, healthcheck |
| `wm` | `wm` | open, close, move, resize, focus, raise, minimize, maximize |
| `input_router` | `input_router` | subscribe |
| `clipboard` | `clipboard` | `clipboard_copy`, `clipboard_paste` |
| `vfs` | `vfs_pool` | read, write, list, stat, mkdir, rename, copy, unlink, truncate, chmod, search, streams, the access journal, `persist`, and the store install, remove and status calls |
| `toolkit` | `toolkit` | theme and component requests |

The vfs calls take an owner pid, which `vfs_pool` accepts only when it is
the sender's own (apps pass `mk_getpid()`). `vfs_pool` serves only a
process holding FileSystem, so an app that uses it needs that bit in its
`Capsule.mk`.

## Building

Apps depend on it by path with the default `runtime` feature, which pulls
in the heap and panic handler from `nonos_libc` and the toolkit runtime.
Host proof crates (`desktop_proofs`, `setup_layout_proofs`,
`image_paint_proofs`) depend on it with `default-features = false`.
`userland/desktop_proofs` includes `runner/{chrome,dispatch,drag,press_part}.rs`
and checks the press grab and drag on the host.
