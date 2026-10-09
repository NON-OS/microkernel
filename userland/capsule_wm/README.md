# capsule_wm

## Role

`capsule_wm` owns window state for every running capsule on the desktop:
geometry, kind (normal / dialog / tooltip / popup), visibility, z-order,
focus, and lifecycle subscriptions. It does **not** own pixels. Apps register
their surface with the kernel registry, share the handle with the compositor
directly, and tell the wm only their (window_id, geometry, kind) so the wm can
answer authoritative questions like "which window owns the point under the
cursor".

```text
app   --SCENE_SUBMIT(handle, rect)-->  compositor
  \                                         ^
   \--OP_WINDOW_OPEN/MOVE/RESIZE-->  wm  --OP_FOCUS_SET-->/
                                          --OP_LIFECYCLE notify subscribers
```

## Microkernel contract

The manifest grants `IPC` and `Memory`, and no `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0x18
```

No driver, IRQ, or DMA authority. Every interaction with peer capsules
rides `mk_ipc_call` and `mk_ipc_send_to_pid` over the standard `NWMP`
envelope.

## IPC surface

Service endpoint: `service:4330:wm`. Reply endpoint: `reply:4331:endpoint.wm.reply`.

| op                     | code | body                                              |
| ---------------------- | ---: | ------------------------------------------------- |
| `OP_HEALTHCHECK`       | 0x01 | empty                                             |
| `OP_WINDOW_OPEN`       | 0x02 | `window_id u32, kind u32, x u32, y u32, w u32, h u32` |
| `OP_WINDOW_CLOSE`      | 0x03 | `window_id u32, _pad u32`                          |
| `OP_WINDOW_MOVE`       | 0x04 | `window_id u32, _pad u32, x u32, y u32`             |
| `OP_WINDOW_RESIZE`     | 0x05 | `window_id u32, _pad u32, w u32, h u32`             |
| `OP_WINDOW_FOCUS`      | 0x06 | `window_id u32, _pad u32`                          |
| `OP_WINDOW_RAISE`      | 0x07 | `window_id u32, _pad u32`                          |
| `OP_LIFECYCLE_SUBSCRIBE` | 0x08 | empty                                           |
| `OP_WINDOW_MINIMIZE`   | 0x09 | `window_id u32, _pad u32`                          |
| `OP_WINDOW_RESTORE`    | 0x0A | `window_id u32, _pad u32`                          |
| `OP_QUERY_TOPMOST`     | 0x0B | `x u32, y u32`; replies with the owner, window and window-local point |
| `OP_ROUTE_FOCUS`       | 0x0C | `owner_pid u32, window_id u32`; the input router only |
| `OP_QUERY_FOCUS`       | 0x0D | empty; replies with the focused owner and window   |
| `OP_WINDOW_MAXIMIZE`   | 0x0E | `window_id u32, flags u32, x u32, y u32, w u32, h u32` |

Every window op acts on the sender's own windows. A frame whose header
`parse` refuses is still answered, with `E_BAD_MAGIC`, `E_BAD_VERSION` or
`E_BAD_LEN` and the op and request id it named (`src/protocol/decode.rs`).

`window_id` is per-pid; the wm keys the table by `(owner_pid, window_id)`.
One pid holds at most `PER_OWNER` windows, half the table, so no client can refuse every
other program a window; past it `OP_WINDOW_OPEN` answers `E_NOMEM` as for a full table
(`src/window/table/insert.rs`, held by `userland/wm_proofs`). The windows and lifecycle
subscriptions of a pid that ended are dropped by the serve loop's sweep
(`src/server/runner/sweep_dead.rs`).

## Stacking and focus

Every raise goes through `z_order::raise`, which takes a new top `z` only
when another window sits above, and the raise, restore, maximize and open
handlers send the compositor `OP_FOCUS_SET` exactly when the order moved.
The compositor stacks layers in the order it hears of raises, so the window
drawn on top is the one `topmost_hit_at` gives the next click to.
`OP_ROUTE_FOCUS`, sent by the input router on a press, goes through
`focus::press_focus`: the window is raised and focused in one step before
the press is delivered. Tooltips take no focus. The rules are held by
`userland/wm_proofs` (`press_tests.rs`) and, end to end with the
compositor and the router, by `userland/desktop_proofs`.

The handbook page is [Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).

## Notifications

Lifecycle subscribers receive an `NWMV` envelope (8-byte header +
20-byte payload: `event_kind u32, owner_pid u32, window_id u32, x u32, y u32`)
on `OP_WINDOW_OPEN` and `OP_WINDOW_CLOSE`. Event kinds: 0 = opened, 1 = closed,
2 = full screen, with `x` 1 when the window now covers the dock's band and
0 when it no longer does.

`OP_WINDOW_MAXIMIZE`'s `flags` word was padding. Bit 0
(`MAXIMIZE_FLAG_FULL_SCREEN`) makes the window full screen: the client's
green button, a rect from the foot of the menubar to the bottom edge. The
window covers the dock's band while it is full screen and not minimised
(`window/full_screen.rs`), and kind 2 is sent each time that changes: on a
maximise or restore with or without the flag, a resize, a minimise, a
restore from the dock, and a window opened again. A close or the end of the
process is the closed event, which ends it too. A client that sends zero
there maximises as before, and a subscriber that knows only kinds 0 and 1
drops kind 2, so the envelope keeps version 1.
