# capsule_driver_ps2_input: surface and release

What the capsule does today, what release still needs, and how it is checked.
The summary is in [README.md](README.md); what only real machines can show, in
[HARDWARE.md](HARDWARE.md).

## Current implemented surface

- Broker discovery for the keyboard and AUX platform records.
- Claim/PIO/IRQ setup for i8042, IRQ1, and IRQ12.
- Broker-only i8042 reads and writes, with no inline port assembly.
- Keyboard scanning enable and stale-output drain before service start.
- AUX enable, controller config update, mouse defaults, and report enable.
- Keyboard bytes framed (E0, Pause, command replies), decoded and posted, with
  nothing kept; Print Screen, Pause and Break keys.
- Caps, Num and Scroll Lock LEDs kept in line with the driver's lock state.
- Standard 3-byte and IntelliMouse 4-byte packet parser with timed resync and
  reset detection, and a mouse queue.
- A mouse that resets brought up again with the keyboard still typing.
- Key, motion and button events posted to the kernel input ring.
- Polling operation for mouse events.
- Controller telemetry that does not consume pending data bytes.
- Diagnostic counters for dropped events and controller/parser errors.

## Release target

A signed i8042 input service that can be spawned by the kernel mirror, claim the
broker records, expose keyboard and AUX mouse events through the stable
endpoint, survive malformed input bytes, report loss and parser damage, and hand
all routing decisions to the userland input stack.

## Release evidence and checklist

Evidence required for release: QEMU keyboard injection reaches the kernel input
ring; QEMU AUX pointer injection reaches `OP_POLL_MOUSE`; teardown logs show
PIO, IRQ1, IRQ12, and both device claims revoked; real legacy PS/2 hardware
records keyboard and mouse events; USB-only hardware boots without requiring
this capsule.

Checklist: the signed manifest and kernel mirror are present; `make -B
nonos-mk-driver-ps2-input` passes; static checks pass for broker-only PIO and
AUX IRQ12 wiring; QEMU keyboard and mouse input validation passes; event
overflow and packet sync-loss counters are observable; real hardware proof is
attached to the release record.

## Explicit non-goals today

This capsule does not implement USB HID, compose keys, international input
methods, accessibility policy, focus routing, screen-lock policy, text
rendering, compositor cursor policy, or persistent input logs.

## Verification

From the repository root, `make -B nonos-mk-driver-ps2-input` and `bash
nonos-ci/run-static-checks.sh`; the host proofs with `cd
userland/ps2_input_proofs && cargo test --release`. Handbook:
[drivers](../../docs/handbook/drivers.md).

## Static checks

`bash nonos-ci/run-static-checks.sh` requires: no raw PIO assembly in the
capsule; broker setup phases roll back partial grants; the controller-status
operation is present; the AUX mouse path is wired through IRQ12, packet
parsing, and `OP_POLL_MOUSE`; each key is posted with `mk_input_event_post`
and no `OP_POLL_EVENTS` returns; the endpoint string `driver.ps2_kbd0`
remains present for compatibility.
