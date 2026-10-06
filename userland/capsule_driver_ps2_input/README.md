# capsule_driver_ps2_input

## Role

`capsule_driver_ps2_input` is the userland owner of the IBM-compatible i8042
input controller. It serves the legacy PS/2 keyboard stream and the AUX mouse
stream from one capsule because both devices share ports `0x60` and `0x64`.
Splitting keyboard and mouse into separate capsules would create two owners for
one controller and would break the microkernel device-authority model.

The public service name remains `driver.ps2_kbd0` for compatibility with older
callers. The endpoint now represents the complete i8042 input source: keyboard
events, AUX mouse events, controller status, and bounded diagnostic counters.

```text
                 MkDeviceList / MkDeviceClaim
                           |
                           v
                 capsule_driver_ps2_input
                   owns i8042 PIO grant
                    ports 0x60 and 0x64
                           |
        +------------------+------------------+
        |                                     |
     IRQ1                                  IRQ12
 keyboard                               AUX mouse
        |                                     |
 scancode decoder                    3-byte packet parser
        |                                     |
 bounded key ring                    bounded mouse ring
        |                                     |
        +------------------+------------------+
                           |
        MkInputEventPost --> kernel input ring --> input_router
        IPC service replies (to the kernel only)
```

## Microkernel contract

The kernel publishes two synthetic platform records:

| Record | Purpose |
|---|---|
| PS/2 keyboard | owns the shared i8042 PIO window and IRQ1 |
| PS/2 AUX | publishes IRQ12 for the same capsule, with no PIO BAR |

The capsule claims both records. Only the keyboard record grants PIO, because
the i8042 data and command ports are shared. The AUX record exists so IRQ12 is
owned, acknowledged, and released through the same broker path as every other
interrupt source.

The capsule uses only microkernel calls:

| Syscall family | Use |
|---|---|
| `MkDeviceList` | discover PS/2 keyboard and AUX platform records |
| `MkDeviceClaim` | bind device authority to this capsule's pid |
| `MkPioGrant` | obtain the i8042 PIO range |
| `MkPioRead` / `MkPioWrite` | access data/status/command ports |
| `MkIrqBind` / `MkIrqAck` | own and acknowledge IRQ1 and IRQ12 |
| `MkInputEventPost` | post each decoded key, motion and button event to the kernel input ring (`src/keymap/post.rs`, `src/mouse/post.rs`) |
| `MkIpcRecv` / `MkIpcSend` | serve the driver endpoint |

No capsule may send to `driver.ps2_kbd0`: the kernel holds the endpoint to an
empty list (`src/services/registry/held.rs`), so only the kernel's own sends
reach it, by name or by pid.

There is no inline port assembly in the capsule. The static gate rejects raw
`in`/`out` instructions so every byte crosses the broker grant checks.

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Irq`, `Pio` and
`InputSource` (`CAPSULE_REQUIRED_CAPS = 0x358018`). It does not grant `Mmio`,
`Dma`, filesystem, graphics, network, admin, debug, or persistent storage
authority.

`InputSource` admits `MkInputEventPost`. The kernel's consumer gate for
`MkInputEventDrain` and `MkInputEventWait` also accepts `InputSource`, so this
capsule could drain the keystroke ring as `input_router` does; nothing in its
code calls those.

```text
allowed:
  - i8042 PIO through MkPio*
  - IRQ1 and IRQ12 through MkIrq*
  - bounded in-memory event rings
  - posts to the kernel input ring through MkInputEventPost
  - IPC replies to the kernel

forbidden:
  - persistent key or mouse logs
  - focus decisions
  - input-method policy
  - window routing
```

## Runtime lifecycle

Startup is deliberately ordered so partial ownership does not survive a failed
phase:

1. Discover the PS/2 keyboard platform record.
2. Claim the keyboard record.
3. Grant the i8042 PIO window.
4. Bind IRQ1.
5. Discover and claim the AUX platform record.
6. Bind IRQ12.
7. Drain stale controller output.
8. Enable keyboard scanning.
9. Enable the AUX port, set the controller config byte, reset mouse defaults,
   and enable mouse reporting.
10. Start the IPC service loop.

PIO or IRQ setup failure rolls back broker grants that were already acquired.
If the mouse enable sequence is not acknowledged, the keyboard service still
runs and status replies report `mouse_enabled = 0`.

## Interface contract

| Operation | Meaning | Reply payload after status |
|---|---|---|
| `OP_HEALTHCHECK` | service liveness | none |
| `OP_POLL_EVENTS` | drain keyboard events | `u32 count`, then `count` keyboard records |
| `OP_POLL_MOUSE` | drain AUX mouse events | `u32 count`, then `count` mouse records |
| `OP_GET_STATE` | diagnostic counters | seven little-endian `u64` counters |
| `OP_CONTROLLER_STATUS` | brokered i8042 snapshot | fixed 28-byte controller payload |

## Wire format

Requests use the `NKBD` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a little-endian signed 32-bit status word.
Status `0` means the operation completed.

## Keyboard Records

`OP_POLL_EVENTS` returns a bounded batch. Each keyboard record is 3 bytes:

| Offset | Type | Meaning |
|---:|---|---|
| 0 | `u8` | scancode byte after decoder handling |
| 1 | `u8` | event flags |
| 2 | `u8` | reserved, always zero |

Overflow is visible through state counters. The capsule drops new events when
the ring is full rather than blocking IRQ handling.

## Mouse Records

`OP_POLL_MOUSE` returns a bounded batch of decoded standard PS/2 mouse packets.
Each mouse record is 8 bytes:

| Offset | Type | Meaning |
|---:|---|---|
| 0 | `i16` | X movement, little-endian |
| 2 | `i16` | Y movement, little-endian, screen-positive upward normalized |
| 4 | `i8` | wheel delta, zero for the base 3-byte protocol |
| 5 | `u8` | button bitset: left, right, middle |
| 6 | `u8` | overflow and parser flags |
| 7 | `u8` | reserved, always zero |

The record keeps the device's wheel sign, where a notch toward the user is
+1. The event posted to the input ring negates it: there a notch away is +1,
as USB and I2C HID mice report it and as every app reads it.

The parser requires bit 3 in the first packet byte for synchronization. Bad
alignment increments `mouse_sync_errors` and never enters the public event
ring.

## State Payload

`OP_GET_STATE` returns these little-endian `u64` counters in order:

| Index | Counter |
|---:|---|
| 0 | keyboard events seen |
| 1 | keyboard events dropped |
| 2 | controller parity errors |
| 3 | controller timeout errors |
| 4 | mouse events seen |
| 5 | mouse events dropped |
| 6 | mouse packet sync errors |

The state payload is diagnostic only. It does not expose raw key history beyond
the bounded event records returned by explicit polling.

## Controller Status Payload

`OP_CONTROLLER_STATUS` reads the status port without reading the data port, so
it cannot consume a pending key or mouse byte. The 28-byte payload is:

| Offset | Type | Meaning |
|---:|---|---|
| 0 | `u8` | raw i8042 status byte |
| 1 | `u8` | output buffer full |
| 2 | `u8` | parity error |
| 3 | `u8` | timeout error |
| 4 | `u32` | queued keyboard events |
| 8 | `u32` | keyboard ring head |
| 12 | `u32` | keyboard ring tail |
| 16 | `u32` | output byte belongs to AUX |
| 20 | `u32` | mouse reporting enabled by setup |
| 24 | `u32` | queued mouse events |

## Privacy and persistence

Keyboard and pointer input is treated as private live data. The capsule keeps
only bounded memory rings and diagnostic counters. It does not write input to
disk, forward input to unrelated services, keep history after process exit, or
decide where input should be delivered.

Focus, compositor routing, lock-screen policy, accessibility policy, compose
keys, and text rendering belong to higher-level userland capsules. The keyboard
layout is the one exception: nothing downstream resolves shift or layout, so
the driver translates each key through the active layout (`nonos_keymap`),
reads the choice from the policy store at most once a second, and cycles it on
Ctrl+Alt+Space (`src/keymap/active.rs`).

## Failure model

| Failure | Behavior |
|---|---|
| keyboard platform record missing | one log line, exit `EXIT_ABSENT` (2), nothing claimed |
| no i8042 answers (the ports float, the output buffer never empties) | the probe gives its claim back, one log line, exit `EXIT_ABSENT` (2) |
| keyboard enable fails | both claims are released (with the port grant and both lines); the attempt is repeated on the shared bounded schedule, and running out exits `EXIT_GAVE_UP` (6) |
| PIO grant denied | startup fails and device claim is released |
| IRQ1 bind denied | PIO grant and device claim are released |
| AUX record or IRQ12 bind denied | keyboard grants are released and startup fails |
| mouse command not acknowledged | keyboard path stays live, mouse status reports disabled |
| key ring full | new keyboard events are dropped and counted |
| mouse ring full | new mouse events are dropped and counted |
| packet sync loss | packet is discarded and counted |

## Current implemented surface

- Broker discovery for the keyboard and AUX platform records.
- Claim/PIO/IRQ setup for i8042, IRQ1, and IRQ12.
- Broker-only i8042 reads and writes, with no inline port assembly.
- Keyboard scanning enable and stale-output drain before service start.
- AUX enable, controller config update, mouse defaults, and report enable.
- Scancode event ring for keyboard input.
- Standard 3-byte PS/2 mouse packet parser and event ring.
- Key, motion and button events posted to the kernel input ring.
- Polling operations for keyboard and mouse events.
- Controller telemetry that does not consume pending data bytes.
- Diagnostic counters for dropped events and controller/parser errors.

## State ownership

The capsule owns the i8042 PIO grant, IRQ1 grant, IRQ12 grant, keyboard decoder
state, the active layout, AUX packet parser state, and both bounded event
rings. It does not own focus, routing, cursor policy, compositor state, or
input persistence. Those decisions are made by higher-level userland capsules.

## Operating rules

- Keep all i8042 access behind `MkPioRead` and `MkPioWrite`.
- Keep IRQ ownership explicit: keyboard on IRQ1, AUX mouse on IRQ12.
- Never persist keyboard or pointer input.
- Never route input to windows from this capsule.
- Never make focus or lock-screen decisions here.
- Drop deterministically on bounded-ring overflow and expose the counter.
- Treat packet sync loss as data-plane damage, not as a kernel fault.

## Release target

The release target for this capsule is a signed i8042 input service that can be
spawned by the kernel mirror, claim the broker records, expose keyboard and AUX
mouse events through the stable endpoint, survive malformed input bytes, report
loss and parser damage, and hand all routing decisions to the userland input
stack.

## Release evidence

Evidence required for release:

- QEMU keyboard injection reaches `OP_POLL_EVENTS`.
- QEMU AUX pointer injection reaches `OP_POLL_MOUSE`.
- Teardown logs show PIO, IRQ1, IRQ12, and both device claims revoked.
- Real legacy PS/2 hardware records keyboard and mouse events.
- USB-only hardware boots without requiring this capsule.

## Release checklist

- Signed manifest and kernel mirror are present.
- `make -B nonos-mk-driver-ps2-input` passes.
- Static checks pass for broker-only PIO and AUX IRQ12 wiring.
- QEMU keyboard and mouse input validation passes.
- Event overflow and packet sync-loss counters are observable.
- Real hardware proof is attached to the release record.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- On a legacy-free machine (no i8042) the probe takes one claim and one release, and the capsule exits `EXIT_ABSENT` without a retry.
- On an EC-emulated i8042 the keyboard types and, where fitted, the touchpad or mouse moves; a wheel mouse scrolls.
- A firmware-disabled aux port is switched back off and the keyboard keeps scanning.

## Explicit non-goals today

This capsule does not implement USB HID, compose keys,
international input methods, accessibility policy, focus routing, screen-lock
policy, text rendering, compositor cursor policy, or persistent input logs.

## Verification

Run these from the repository root:

```sh
make -B nonos-mk-driver-ps2-input
bash nonos-ci/run-static-checks.sh
```

The static checks require:

- no raw PIO assembly in the capsule;
- broker setup phases roll back partial grants;
- the controller-status operation is present;
- the AUX mouse path is wired through IRQ12, packet parsing, and `OP_POLL_MOUSE`;
- the endpoint string `driver.ps2_kbd0` remains present for compatibility.

## Evidence still required

Before this driver is called hardware-complete, the remaining evidence is:

- QEMU keyboard scancode injection reaches `OP_POLL_EVENTS`;
- QEMU AUX pointer injection reaches `OP_POLL_MOUSE`;
- teardown logs show PIO, IRQ1, IRQ12, and both device claims revoked;
- a real machine with legacy PS/2 input records keyboard and mouse events;
- a USB-only machine proves the absence of this capsule does not block the USB
  HID path.

Handbook: [drivers](../../docs/handbook/drivers.md).
