# capsule_desktop_shell

## Role

`capsule_desktop_shell` is the userland desktop chrome capsule. It owns
two full-screen surfaces: the desk at `z` 1, under every application
window (which the compositor stacks at `z` 2), with the desktop icons;
and the chrome at `z` 3 000 000, over every window, with the menu bar,
the dock, the tray, menus, the Launchpad, toasts and dialogs. The
layering, input grabs and work area are described in
[0.9.2-notes.md](../../docs/handbook/desktop/0.9.2-notes.md).
It consumes the compositor, wm and input router services and coordinates
with the wallpaper and market capsules. No graphics hardware is touched
directly. The handbook page is
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).

```text
desktop_shell (this capsule)
    |
    | desk surface (z = 1) and chrome surface (z = 3 000 000), full-screen
    v
compositor --> driver.virtio_gpu0 --> display
    ^
    |
wm / wallpaper / market (peer IPC)
```

## Microkernel contract

- `MkIpcRecv` on port `4410` reads tray/spotlight/notify requests
  from app capsules.
- `MkIpcCall` against the compositor, wm, input router, wallpaper,
  market and vfs.
- `MkMmap` allocates the overlay backing buffer, sized from the
  compositor's `OP_DISPLAY_INFO`.
- `MkSurfaceRegister` / `MkSurfaceShare` publish the overlay.
- `MkSpawnInstance` opens another window of an app from the dock.
- `MkPidAlive` finds tray items whose clients ended.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_TRAY_REGISTER` | 0x0002 | app registers a tray entry |
| `OP_TRAY_UPDATE` | 0x0003 | change a tray entry's label |
| `OP_TRAY_REMOVE` | 0x0004 | drop a tray entry |
| `OP_NOTIFY` | 0x0005 | post a transient notification |
| `OP_SPOTLIGHT_OPEN` | 0x0006 | open the Launchpad and its search, or close it |
| `OP_OPEN_WITH` | 0x0007 | open a file in another app (the file manager uses it for images); only a path starting with `/` is taken |
| `OP_TAKE_OPEN_ARG` | 0x0008 | the opened app takes the file it was opened with; a Terminal window takes a tool tile's `run:` or `type:` command line instead (`state/open_arg.rs`) |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x100105D` and `CAPSULE_OPTIONAL_CAPS := 0x100`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0004 | Network | ask `net.dhcp.client` its lease for the tray indicator |
| 0x0008 | IPC | recv on 4410 + send to peers, the clipboard among them for Ctrl+V in the rename box and the Launchpad search |
| 0x0010 | Memory | overlay backing + tray + notify tables |
| 0x0040 | FileSystem | list the root and ask the store's state through vfs (`src/vfs_client`); vfs serves only a holder of FileSystem |
| 0x0100 | Debug (optional) | the frame-time counter's lines and the stuck-setup line; only a `capsule-serial-debug` build grants it |
| 0x1000 | GraphicsSurfaceCreate | register the overlay surface |
| 0x1000000 | SpawnWindow | open another window of an app |

No display query: the compositor gives the shell its size. No driver or
crypto cap.

## Privacy posture

| Invariant | How `capsule_desktop_shell` honors it |
|---|---|
| NO LOGS | Debug is optional, so the `MkDebug` lines (a stuck setup in `wait_for_setup.rs`, the `shell-frametime` counter) reach serial only on a build with `capsule-serial-debug`. The `microkernel-desktop-base` feature set, and so `microkernel-desktop-gui`, turns that feature on today. Spawn `debug_tag` empty. |
| NO TRACES | No notification history past visible TTL. No tray persistence. No spotlight query log. |
| EPHEMERAL | Zero files. Overlay backing in private anonymous mmap that vanishes on exit. |
| NOT LINUX | NONOS Mk-tag syscall ABI; wire is the NDSH header, not D-Bus or systemd-shaped. |
| PRIVACY MICROKERNEL | Seven required bits. A compromise stays bounded to the overlay surface + peer IPC channels. |

## Runtime lifecycle

1. `_start` initializes heap.
2. `setup::prime::run()`:
   - `peers::resolve()` finds the compositor, input router, wm and
     wallpaper, and the market if it is there.
   - `overlay::allocate()` asks the compositor for the canvas size and
     mmaps the backing.
   - `register::register_overlay()` publishes surface + scene_submit at z=1.
   - Healthchecks each peer; sets wallpaper policy.
   - `open_chrome_windows` registers the dock with the wm as a popup
     window, so the hit test sends clicks on it to the shell.
   `wait_for_setup` retries the whole setup every 250 ms until it holds.
3. Server loop drains IPC, refreshes the clock, retries its input and wm
   subscriptions, removes ended clients' tray items, and repaints the
   overlay on damage.

## Failure model

- Peer lookup or surface registration fails at startup: setup is retried
  every 250 ms.
- Tray table full: `E_NOMEM`; app retries. One client holds at most half the
  tray (`PER_OWNER`), and past it is answered the same way.
- A tray app that ended without removing its items: removed every two
  seconds while the loop turns, and at once when a register finds the tray
  full (`server/reap_tray.rs`, `state/tray/ended.rs`).
- Notification flood: bounded queue; oldest dropped.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + IPC loop | `server/runner/*.rs`, `wait_for_setup.rs` |
| Per-op handlers | `server/handlers/*.rs` |
| Tray table | `state/tray/{mod,entry,table}.rs` |
| Input grabs | `state/grab_rule.rs`, `server/grabs.rs` |
| Context | `state/context.rs` |
| Setup peers resolution | `setup/prime/peers.rs` |
| Overlay mmap | `setup/prime/overlay.rs` |
| Surface register + scene_submit | `setup/prime/register.rs` |
| Setup driver | `setup/prime/run/*.rs` |
| Compositor client | `compositor_client/*.rs` |
| wm / wallpaper / market / vfs clients | `wm_client/`, `wallpaper_client/`, `market_client/`, `vfs_client/` |
| Render (chrome paint) | `render/*.rs` |
| Wire protocol | `protocol/*.rs` |

## Wire format

20-byte header followed by a typed payload, magic `0x4E44_5348`
("NDSH"), version 1.

## State ownership

`Context` owns: peer ports, display dims, the desk and chrome backings,
the grabs it holds, the tray table, the dock's window tracking, toasts,
next request id.

## What the chrome says

- The menu bar draws the tray's labels (`OP_TRAY_REGISTER`) between the menu
  titles and the status cluster; labels that do not fit are counted in a
  "+N" (`render/topbar/tray.rs`, `state/tray/fit.rs`). A label is status
  text, not a control.
- The battery glyph and percent are drawn only for a reading the kernel
  gives. `MkBatteryStatus` refuses on every machine today (the kernel has no
  AML interpreter to read ACPI `_BST`), so no battery is shown rather than
  "AC". There is no unread-notification dot: a notification is its toast.
- A launch that opens nothing says so, from the dock, a menu, the Launchpad,
  a dialog or Ctrl+Alt+Esc: "<App> turned off at setup", Qwen's own reason,
  or "<App> did not open" (`apps_off/open.rs`). A tool tile hands the
  Terminal its command line (`state/open_arg.rs`). `pastel` runs at once,
  since it prints its help with no arguments. Every other tool is typed
  with the cursor after it, ready for its arguments. Open With answers the
  file manager with an error when the app could not be started, and drops
  the path it held for it.
- A refused package says why in words ("Package: signature or digest failed
  verification"), and a desktop New, Rename, Move or Delete the file service
  refused gives its reason ("Could not delete: the folder is not empty"),
  `state/says.rs`. Deleting a folder removes it with everything in it, as
  the prompt says.

## Operating rules

- No `unsafe` past `_start` and `mk_mmap`.
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- One function per file where non-trivial; `mod.rs` re-exports only.

## Release target

x86_64-nonos-user.

## Release evidence

`cargo check --features microkernel-core,nonos-production,nonos-capsule-desktop-shell`
must compile clean.

## Release checklist

- [x] 15-line license header on every file
- [x] `Capsule.mk` mask `0x100105D`, Debug optional
- [x] Spawn grants no bit the mask lacks
- [x] Kernel mirror at `src/userspace/capsule_desktop_shell/`
- [x] Cert + manifest baked
- [x] Spawn wired
- [ ] QEMU spawn-verify (blocked by OVMF #PF)

## Explicit non-goals today

- Search covers the Launchpad's apps, tools and installed programs, not
  files or the content of other capsules.
- No tray icons beyond text label (deferred until toolkit images).
- No notification persistence across restart.
- One global chord only: Ctrl+Alt+Esc, reserved by the input router,
  brings Process Manager forward.

## Verification

- `nonos-ci/run-static-checks.sh` clean (desktop shell policy
  ownership markers live in userland; render path routes through
  compositor IPC; kernel source free of desktop-shell state markers).
- `make nonos-mk-host-trust-verify` verifies
  the baked `desktop_shell.manifest.bin`.
- Kernel cargo check matrix passes with `nonos-capsule-desktop-shell`.
- Host proofs: `userland/desktop_proofs` (`tray_share_tests.rs` holds the
  per-client share and the removal of ended clients' items on the real
  tray table).
