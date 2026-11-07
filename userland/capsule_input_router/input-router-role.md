# capsule_input_router: Role

Part of [capsule_input_router](README.md).

## Role

`capsule_input_router` is the userland input dispatcher. It drains the kernel
input ring with `MkInputEventDrain`, moves the cursor, asks the window manager
which window is under the pointer or holds keyboard focus, and delivers each
event to that window's process. No driver claims live here: this capsule is IPC
plus the `MkInputEvent*` calls.

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

The handbook page is [Compositor](../../docs/handbook/desktop/compositor.md),
section "Input routing".

## Microkernel contract

- `MkInputEventDrain` reads up to 32 events a pass from the kernel input ring;
  `MkInputEventWait` parks for at most `INPUT_WAIT_MS`, 8 ms, when the ring is
  empty.
- `MkIpcRecv` on port `4320` reads subscription and grab requests.
- `MkIpcCall` asks `wm` and `policy` (mouse sensitivity and screen timeout,
  re-read at most every two seconds), sends the compositor cursor updates,
  and tells it the idle blank (`IDLE_BLANK`).
- `MkIpcSendToPid` delivers each event to its target process.
- `MkPidAlive` drops the subscriptions, grabs and idle inhibits of processes
  that ended.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_SUBSCRIBE` | 0x0002 | register the sender with a mask of the input kinds it takes |
| `OP_GRAB_REQUEST` | 0x0003 | take every event of some kinds; only the five services in `GRABBERS` may, and only while focused (`route/grab_rule.rs`) |
| `OP_GRAB_RELEASE` | 0x0004 | give the grab back |
| `OP_TRUSTED_HOLD` | 0x0005 | from the compositor only, one-way and never answered: the pid holding its trusted band (0: none); a hold not heard again for 3 s lapses (`route/trusted_hold.rs`) |
| `OP_IDLE_INHIBIT` | 0x0007 | the caller holds the screen awake (u32 1) or lets it go (0), as `zwp_idle_inhibit_manager_v1` asks while a surface that wants it is visible; any client, at most 16 at once (`E_BUSY` past that), let go when the process ends; E_INVAL for any other body. It holds back the screen blank only and never wakes a blank screen (`state/idle/`) |
| `OP_DISPLAY_SIZE` | 0x0006 | from the compositor only, one-way and never answered: its display's width and height (u32 each), on every change and every 5 s; the pointer's edges move at once. Until the size is known, by this or by the compositor's DISPLAY_INFO, no pointer event is routed (`state/cursor_edges.rs`) |
| `OP_SESSION_LOCK` | 0x0008 | from the shell only (`E_ACCES` otherwise): body u32, 1 lock, 0 unlock, 2 ask, then up to four rectangles (x, y, width, height, u32 each) where the pointer is drawn while locked, hidden anywhere else; answered with the status and the state after it (u32, 1 locked). While locked every event goes to the shell, over focus, grabs and the trusted hold; a key's release goes to the process that saw it go down, and nothing else reaches a window. The lock has no lease: it lasts until the shell clears it, and a shell that ends while locked leaves it set for its successor, which asks (`route/session_lock.rs`) |

Super+L (`route/chord.rs`, `is_lock_chord`) goes to the shell over focus, as
Ctrl+Alt+T, Alt+Tab and Ctrl+Alt+Shift+F do.

Auto-lock: with the policy's auto-lock time set (minutes, 0 never, read
with the screen timeout), the router sends the shell a notice (delivery
notice code 2, `NOTICE_IDLE_LOCK`) once that long passed with no input, once
for each stretch without input, and never while the session is locked. An
idle inhibit holds it back only when the policy's `LockInhibitHolds` is set;
otherwise it holds back the blank alone. The shell locks if a passphrase is
set (`state/idle/auto_lock.rs`, input_proofs `router_auto_lock_tests`).

Idle: every input event the router takes, from any source, routed or
refused, restarts the idle count. With the policy's screen timeout set
(minutes, read with the mouse sensitivity every two seconds; 0 never) and no
inhibit held, the router tells the compositor `IDLE_BLANK` 1 once that long
passed with no input, and 0 on the next input, which is routed as usual. A
refused or unanswered word is sent again after a second; a changed one at
once (`state/idle/`, input_proofs `router_idle_tests`).

`GRABBERS` is the boot splash, the setup wizard, the input probe, the desktop
shell and the installer. Each may grab only while its window has focus, the
shell while the desktop has it; with no window manager answering (setup, the
installer) the name alone decides. A grab holder that cannot receive loses its
grab. A
frame whose header `parse` refuses is answered with `E_BAD_MAGIC`,
`E_BAD_VERSION` or `E_BAD_LEN` and the op and request id it named.

## Routing

- Pointer: the cursor moves first, then `OP_QUERY_TOPMOST` asks the window
  manager for the window under it. A press on a window arms a press grab
  (`state/press.rs`) so the drag and the release go to the same window in the
  same frame, and sends `OP_ROUTE_FOCUS`, which raises and focuses the window
  before the press is delivered. Any press drops the cached hover window. A
  press on no window, or on the shell, goes to the desktop shell.
- Keys: a key press goes to the pid `OP_QUERY_FOCUS` names; its release goes to
  the pid that got the press.
- Every delivery needs a subscription whose kind mask allows the event.
- Reserved keys, the trusted band, Bluetooth pairing and gamepads come first:
  [Routing ahead of focus](input-router-ahead-of-focus.md).
