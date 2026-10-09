# capsule_input_router

## Role

`capsule_input_router` is the userland input dispatcher. It drains the
kernel input ring with `MkInputEventDrain`, moves the cursor, asks the
window manager which window is under the pointer or holds keyboard
focus, and delivers each event to that window's process. No driver
claims live here: this capsule is IPC plus the `MkInputEvent*` calls.

```text
driver.ps2_kbd0 / driver.usb_hid0 / driver.i2c_hid0
    |
    | kernel input ring (MkInputEventPost from the driver side)
    v
capsule_input_router (this capsule) --OP_QUERY_TOPMOST / OP_ROUTE_FOCUS / OP_QUERY_FOCUS--> wm
    |                                --OP_CURSOR_UPDATE--> compositor
    | NINP delivery (8-byte header + 32-byte InputEvent)
    v
target window (terminal / calculator / desktop_shell / ...)
```

The handbook page is [Compositor](../../docs/handbook/desktop/compositor.md), section "Input routing".

## Microkernel contract

- `MkInputEventDrain` reads up to 32 events a pass from the kernel input
  ring; `MkInputEventWait` parks for at most `INPUT_WAIT_MS`, 8 ms, when
  the ring is empty.
- `MkIpcRecv` on port `4320` reads subscription and grab requests.
- `MkIpcCall` asks `wm` and `policy` (mouse sensitivity, re-read at most
  every two seconds) and sends the compositor cursor updates.
- `MkIpcSendToPid` delivers each event to its target process.
- `MkPidAlive` drops the subscriptions and grabs of processes that ended.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_SUBSCRIBE` | 0x0002 | register the sender with a mask of the input kinds it takes |
| `OP_GRAB_REQUEST` | 0x0003 | take every event of some kinds; only the five services in `GRABBERS` may |
| `OP_GRAB_RELEASE` | 0x0004 | give the grab back |

`GRABBERS` is the boot splash, the setup wizard, the input probe, the
desktop shell and the installer. A grab holder that cannot receive loses
its grab. A frame whose header `parse` refuses is answered with
`E_BAD_MAGIC`, `E_BAD_VERSION` or `E_BAD_LEN` and the op and request id it
named.

## Routing

- Pointer: the cursor moves first, then `OP_QUERY_TOPMOST` asks the window
  manager for the window under it. A press on a window arms a press grab
  (`state/press.rs`) so the drag and the release go to the same window in
  the same frame, and sends `OP_ROUTE_FOCUS`, which raises and focuses the
  window before the press is delivered. Any press drops the cached hover
  window. A press on no window, or on the shell, goes to the desktop shell.
- Keys: a key press goes to the pid `OP_QUERY_FOCUS` names; its release
  goes to the pid that got the press.
- Every delivery needs a subscription whose kind mask allows the event.

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x200018`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x08 | IPC | recv on 4320, calls to wm, policy and compositor, deliveries |
| 0x10 | Memory | subscription, grab and routing tables |
| 0x200000 | InputSource | drain and wait on the raw-input ring |

`Debug` is absent. No Driver, Mmio, Irq, Dma or Pio: this capsule only
consumes the event ring the drivers post to.

## Privacy posture

| Invariant | How `capsule_input_router` honors it |
|---|---|
| NO LOGS | No Debug cap; spawn `debug_tag` empty; no `MkDebug` calls, so keystrokes never reach serial. |
| NO TRACES | No event history kept. Each event is drained, routed and dropped. |
| EPHEMERAL | Zero files. The subscription table is bounded by `MAX_SUBSCRIBERS`, 64. |
| NOT LINUX | NONOS Mk syscall ABI. Requests use the `NIRS` header, deliveries the `NINP` frame; not evdev or X11. |
| PRIVACY MICROKERNEL | Three-bit cap mask. No Network, FileSystem, Crypto, Graphics or Driver cap. |

## Runtime lifecycle

1. `_start` initialises the heap and the context (subscriptions, grabs,
   cursor, press and hover state, key targets).
2. The loop drains IPC requests, drains up to 32 input events and routes
   each, sends the compositor a cursor update when the pointer moved, and
   parks in `MkInputEventWait` when the ring is empty.

## Failure model

- No subscription allows the event: it is dropped.
- Target cannot receive: the delivery fails; a grab holder that cannot
  receive loses its grab.
- Window manager does not answer: the event has no window target and
  pointer events fall to the desktop shell.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + loop | `server/runner.rs`, `server/drain_ipc.rs` |
| Per-op handlers | `server/handlers/{health,subscribe,grab_request,grab_release}.rs` |
| Reply builder | `server/respond.rs` |
| Kernel ring | `sources/kernel_ring.rs` |
| Routing | `route/{dispatch,keyboard,deliver}.rs`, `route/pointer/*.rs` |
| State | `state/{cursor,press,hover,key_targets}.rs`, `state/grabs/*.rs`, `state/subscriptions/*.rs`, `state/context/*.rs` |
| Clients for wm, compositor, policy | `clients/*` |
| Wire protocol | `protocol/*.rs` |

## Wire format

Requests: 20-byte header, magic `0x4E49_5253` ("NIRS"), version 1, op at
offset 6, flags at 8, request id at 12, payload length at 16. Deliveries:
8-byte header with magic `0x4E49_4E50` ("NINP") followed by the 32-byte
`InputEvent` (kind u16, flags u16, code u32, x, y, delta_x, delta_y as
i32, timestamp u64), the shape app_skeleton reads.

## Release checklist

- [x] `Capsule.mk` with `CAPSULE_REQUIRED_CAPS = 0x200018`
- [x] Capability mask audited (3 bits, no Debug)
- [x] Kernel mirror at `src/userspace/capsule_input_router/`
- [x] Cert + manifest baked into `nonos-data/trust/capsules/`
- [x] Spawn wired through `src/userspace/init/spawn_plan/`
- [x] No keystroke ever reaches a log surface

## Explicit non-goals today

- No IME or dead-key composition.
- No keyboard macro expansion.
- No global hotkey table. `capsule_desktop_shell` owns global shortcuts.
- No input recording for replay.

## Verification

- `userland/desktop_proofs/` includes `state/press.rs` and runs one press
  through the router's press grab, the window manager and the compositor
  scene together.
- `make nonos-mk-host-trust-verify` verifies
  the baked `input_router.manifest.bin` against the trust anchor.
