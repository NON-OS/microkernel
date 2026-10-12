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
- `MkIpcCall` asks `wm` and `policy` (mouse sensitivity, re-read at most every
  two seconds) and sends the compositor cursor updates.
- `MkIpcSendToPid` delivers each event to its target process.
- `MkPidAlive` drops the subscriptions and grabs of processes that ended.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_SUBSCRIBE` | 0x0002 | register the sender with a mask of the input kinds it takes |
| `OP_GRAB_REQUEST` | 0x0003 | take every event of some kinds; only the five services in `GRABBERS` may |
| `OP_GRAB_RELEASE` | 0x0004 | give the grab back |

`GRABBERS` is the boot splash, the setup wizard, the input probe, the desktop
shell and the installer. A grab holder that cannot receive loses its grab. A
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
