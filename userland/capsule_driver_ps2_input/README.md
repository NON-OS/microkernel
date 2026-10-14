# capsule_driver_ps2_input

## Role
The userland owner of the i8042: the PS/2 keyboard and the AUX mouse share
ports `0x60` and `0x64`, so one capsule serves both, as `driver.ps2_kbd0`. Keys
and LEDs: [INPUT.md](INPUT.md); operations: [PROTOCOL.md](PROTOCOL.md).

```text
MkDeviceList / MkDeviceClaim --> capsule_driver_ps2_input (i8042 PIO grant)
  IRQ1 keyboard -> framer, decoder; IRQ12 AUX mouse -> packet parser, queue
  MkInputEventPost --> kernel input ring --> input_router
```

## Microkernel contract
Two platform records, the keyboard with the PIO window and IRQ1, the AUX with
IRQ12; only `MkDevice*`, `MkPio*`, `MkIrq*`, `MkInputEventPost` and `MkIpcRecv`
/ `MkIpcSend`, no port assembly ([CONTRACT.md](CONTRACT.md)).

## Interface contract
`OP_HEALTHCHECK`, `OP_POLL_MOUSE`, `OP_GET_STATE`, `OP_CONTROLLER_STATUS`; op 2
went with the scancode ring and is not reused ([PROTOCOL.md](PROTOCOL.md)).

## Authority
`CAPSULE_REQUIRED_CAPS = 0x358018`: IPC, Memory, Driver, DeviceEnum, Irq, Pio,
InputSource; no Mmio, Dma or storage ([CONTRACT.md](CONTRACT.md#authority)).

## Privacy and persistence
No key is kept once posted; a mouse queue emptied every pass, and counters
([PRIVACY.md](PRIVACY.md#privacy-and-persistence)).

## Runtime lifecycle
Claim, grant, bind both lines, bring the keyboard and the mouse up, serve; a
mouse that resets is brought up again ([LIFECYCLE.md](LIFECYCLE.md)).

## Failure model
No i8042: `EXIT_ABSENT` (2); a keyboard that never comes up: `EXIT_GAVE_UP` (6);
a failing mouse leaves the keyboard working
([LIFECYCLE.md](LIFECYCLE.md#failure-model)).

## Current implemented surface
Discovery, the bring-up, framed and decoded keys, the lock LEDs, the packet
parser, events posted, telemetry ([RELEASE.md](RELEASE.md)).

## Wire format
The `NKBD` header, version `1`, in the 20-byte driver envelope; replies start
with a little-endian signed 32-bit status word, `0` for done.

## State ownership
The grants, the decoder and lock state, the layout, the parser and the queue;
focus and routing belong above ([PRIVACY.md](PRIVACY.md)).

## Operating rules
Every port access through `MkPio*`, explicit IRQ ownership, no stored input,
no routing or focus decisions here ([PRIVACY.md](PRIVACY.md)).

## Release target
A signed i8042 input service the kernel mirror spawns, handing every routing
decision to the userland input stack ([RELEASE.md](RELEASE.md#release-target)).

## Release evidence
QEMU injection end to end, teardown revocation, real PS/2 hardware
([RELEASE.md](RELEASE.md), [HARDWARE.md](HARDWARE.md)).

## Release checklist
Signed manifest and mirror, the build, the static checks, QEMU input, hardware
proof ([RELEASE.md](RELEASE.md#release-evidence-and-checklist)).

## Explicit non-goals today
USB HID, compose keys, input methods, accessibility, focus routing, screen
lock, text rendering, cursor policy, persistent input logs.

## Verification
`make -B nonos-mk-driver-ps2-input`, the static checks and `ps2_input_proofs`
([RELEASE.md](RELEASE.md#verification)).
