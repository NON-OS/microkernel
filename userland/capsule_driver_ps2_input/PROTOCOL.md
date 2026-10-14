# capsule_driver_ps2_input: operations and payloads

What `driver.ps2_kbd0` answers. The contract is in [README.md](README.md).
Requests use the `NKBD` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a little-endian signed 32-bit status
word; status `0` means the operation completed.

| Operation | Meaning | Reply payload after status |
|---|---|---|
| `OP_HEALTHCHECK` | service liveness | none |
| `OP_POLL_MOUSE` | drain AUX mouse events | `u32 count`, then `count` mouse records |
| `OP_GET_STATE` | diagnostic counters | seven little-endian `u64` counters |
| `OP_CONTROLLER_STATUS` | brokered i8042 snapshot | fixed 16-byte controller payload |

Op 2 (`OP_POLL_EVENTS`) drained a ring that held the last 255 scan code bytes
typed, whose only reader, the kernel's `poll_events`, was never called. The
ring is gone (plan/50 C8: no input is kept once delivered), op 2 is answered
`E_INVAL`, and its number is not reused.

## Mouse records

`OP_POLL_MOUSE` returns a bounded batch of decoded standard PS/2 mouse
packets. Each mouse record is 8 bytes: an `i16` X movement (offset 0) and an
`i16` Y movement, screen-positive upward normalized (2), both little-endian;
an `i8` wheel delta, zero for the base 3-byte protocol (4); a `u8` button
bitset, left, right, middle (5); a `u8` of overflow and parser flags (6); a
`u8` reserved, always zero (7). The record keeps the device's wheel sign,
where a notch toward the user is +1. The event posted to the input ring
negates it: there a notch away is +1, as USB and I2C HID mice report it and
as every app reads it. An event is cleared from the queue as it is handed on.

## State payload

`OP_GET_STATE` returns these little-endian `u64` counters in order:

| Index | Counter |
|---:|---|
| 0 | keyboard bytes seen |
| 1 | key events the kernel input ring refused |
| 2 | controller parity errors |
| 3 | controller timeout errors |
| 4 | mouse events seen |
| 5 | mouse events dropped |
| 6 | mouse packet sync errors |

The state payload is diagnostic only: counts, never key history.

## Controller status payload

`OP_CONTROLLER_STATUS` reads the status port without reading the data port,
so it cannot consume a pending key or mouse byte. The 16-byte payload is:

| Offset | Type | Meaning |
|---:|---|---|
| 0 | `u8` | raw i8042 status byte |
| 1 | `u8` | output buffer full |
| 2 | `u8` | parity error |
| 3 | `u8` | timeout error |
| 4 | `u32` | output byte belongs to AUX |
| 8 | `u32` | mouse reporting enabled by setup |
| 12 | `u32` | queued mouse events |
