# capsule_input_router: Authority

Part of [capsule_input_router](README.md).

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x1000000018`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x08 | IPC | recv on 4320, calls to wm, policy and compositor, deliveries |
| 0x10 | Memory | subscription, grab and routing tables |
| 0x1000000000 | InputConsumer | drain and wait on the raw-input ring, which nothing else may (K8) |

`Debug` is absent. No Driver, Mmio, Irq, Dma or Pio: this capsule only consumes
the event ring the drivers post to.

## Privacy posture

| Invariant | How `capsule_input_router` honors it |
|---|---|
| NO LOGS | No Debug cap; spawn `debug_tag` empty; no `MkDebug` calls, so keystrokes never reach serial. |
| NO TRACES | No event history kept. Each event is drained, routed and dropped. |
| EPHEMERAL | Zero files. The subscription table is bounded by `MAX_SUBSCRIBERS`, 64. |
| NOT LINUX | NONOS Mk syscall ABI. Requests use the `NIRS` header, deliveries the `NINP` frame; not evdev or X11. |
| PRIVACY MICROKERNEL | Three-bit cap mask. No Network, FileSystem, Crypto, Graphics or Driver cap. |

## Runtime lifecycle

1. `_start` initialises the heap and the context (subscriptions, grabs, cursor,
   press and hover state, key targets).
2. The loop drains IPC requests, drains up to 32 input events and routes each,
   sends the compositor a cursor update when the pointer moved, and parks in
   `MkInputEventWait` when the ring is empty.

## Failure model

- No subscription allows the event: it is dropped.
- Target cannot receive: the delivery fails; a grab holder that cannot receive
  loses its grab.
- Window manager does not answer: the event has no window target and pointer
  events fall to the desktop shell.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + loop | `server/runner.rs`, `server/drain_ipc.rs` |
| Per-op handlers | `server/handlers/{health,subscribe,grab_request,grab_release}.rs` |
| Reply builder | `server/respond.rs` |
| Kernel ring | `sources/kernel_ring.rs` |
| Routing | `route/{dispatch,keyboard,deliver}.rs`, `route/pointer/*.rs` |
| Bluetooth pairing on the secure path | `route/{bond,refuse,notice}.rs` |
| State | `state/{cursor,press,hover,key_targets}.rs`, `state/grabs/*.rs`, `state/subscriptions/*.rs`, `state/context/*.rs` |
| Clients for wm, compositor, policy | `clients/*` |
| Wire protocol | `protocol/*.rs` |

## Wire format

Requests: 20-byte header, magic `0x4E49_5253` ("NIRS"), version 1, op at offset
6, flags at 8, request id at 12, payload length at 16. Deliveries: 8-byte header
with magic `0x4E49_4E50` ("NINP") followed by the 32-byte `InputEvent` (kind
u16, flags u16, code u32, x, y, delta_x, delta_y as i32, timestamp u64), the
shape app_skeleton reads; the header's flags are in [Routing ahead of
focus](input-router-ahead-of-focus.md#notices).

## Release checklist

- [x] `Capsule.mk` with `CAPSULE_REQUIRED_CAPS = 0x1000000018`
- [x] Capability mask audited (3 bits, no Debug)
- [x] Kernel mirror at `src/userspace/capsule_input_router/`
- [x] Cert + manifest baked into `nonos-data/trust/capsules/`
- [x] Spawn wired through `src/userspace/init/spawn_plan/`
- [x] No keystroke ever reaches a log surface
