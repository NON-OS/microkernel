# PS/2 authority, limits and checks

What `capsule_driver_ps2_input` may do and keeps, what it does not do, and
how it is checked. What it drives is in [PS/2 keyboard and mouse](ps2.md).

## Authority and privacy

The [capsule](../../overview/glossary-boot-root-record.md#capsule) manifest asks for the
[capabilities](../../overview/glossary-boot-root-record.md#capability) IPC, Memory, DeviceEnum,
Driver, Irq, Pio and InputSource and nothing else: no MMIO, no DMA and no Debug
(`CAPSULE_REQUIRED_CAPS`, `userland/capsule_driver_ps2_input/Capsule.mk:18`).
The kernel installs the set the signed manifest names, and its spawn site asks
for nothing beyond it (`requested_caps`,
`src/hardware/ps2_kbd_capsule/spawn.rs:51-57`). Without Debug the driver's own
`[driver_ps2]` lines are refused by the kernel and never reach the console. When
the driver ends with a status other than 0, the kernel prints an `[EXIT]` line
with its service name and status, and for status 2 or 6 the reason in words
(`words`, `src/process/exit/end_rule.rs:38-44`).

No capsule may send to `driver.ps2_kbd0`: the kernel holds it to an empty list
(`KERNEL_ONLY`, `src/services/registry/held_table.rs:27`). The driver keeps no
scan code once its key is posted; key events stay in the kernel
input ring until the router delivers them. The driver writes nothing to disk.

## Limits

- Num Lock lights its LED, and starts lit, but is not applied: a keypad key
  always posts its own code, so the keypad always types digits (`character`,
  `userland/capsule_driver_ps2_input/src/keymap/keypad.rs:43-48`).
- The driver adds no absolute mode, tap or multi-finger gesture of its own for a
  PS/2 touchpad, since it speaks no vendor protocol. What the touchpad sends as
  a mouse is what arrives.
- The driver reads at most 16 bytes from the controller per pass
  (`MAX_BYTES_PER_DRAIN`,
  `userland/capsule_driver_ps2_input/src/poll/drain.rs:26`).
- A mouse that resets more than three times in a minute is turned off for the
  session ([PS/2 pointer](ps2-pointer.md#a-mouse-that-resets)).

## How this is checked

- The [proof crate](../../overview/glossary-peer-list.md#proof-crate) `ps2_input_proofs`
  runs the presence probe, the bring-up sequence and its fallbacks, the keyboard
  and mouse setup, the key framing, the lock LEDs and the bring-up of a mouse
  that reset against a modelled i8042 that records every byte the driver sends:
  59 tests. At commit bff12b97 it held 38, and the check
  `proofs-ps2_input_proofs` passed.
- `input_proofs` holds the packet decode and the layout tables against hostile
  bytes: 96 tests, check `proofs-input_proofs` passes.
- The QEMU run target gives the guest its keyboard and mouse through the q35
  machine's i8042 (`QEMU_USB`, `mk/10-qemu.mk:99-102`).
- For the PS/2 keyboard with its layouts and the volume keys there is one
  real-hardware report. Works on an x86_64 laptop (Intel Gemini Lake, 8 GB),
  maintainer hardware report, 6 October 2026; the image commit was not recorded.

## See also

- [PS/2 keyboard and mouse](ps2.md)
- [PS/2 bring-up](ps2-bringup.md)
- [PS/2 keys](ps2-input.md)
- [PS/2 pointer](ps2-pointer.md)
- [Support matrix](../../hardware/MATRIX.md)
