# bga_proofs

Host proofs for the Bochs Graphics Adapter capsule (`capsule_driver_bga`).
The crate includes the shipping driver source with `#[path]` and runs it
against a register window and video memory in memory (`nonos_devmodel`),
with `libc_shim` standing in for the broker and the kernel, without a boot
and without an adapter. The shim records the DISPI block at every give-back
and poisons the register mapping from its unmap to the release (`seal`), so
a register written after its mapping is given back is seen. The proofs'
`regs` is the shipping file with each register write handed to the test's
adapter model as it is made (`hook`), so a model that answers a write, as an
adapter clamping a mode does, needs no thread and no race.

## What it proves

67 `#[test]` functions.

- **DISPI:** the mode set's settled registers, never GETCAPS (`mode_tests`,
  `mode_screen_tests`; a memory window cannot witness the write order), the
  register offsets (`offset_tests`), the mode read back, panned modes and
  other depths refused, a wider line layout kept at its stride
  (`current_tests`), the DISPI ID accepted only from 0xB0C0 to 0xB0C5 and a
  mode the adapter does not hold not read back as set (`bring_up/check_tests`).
- **EDID and the start-up mode:** the EDID read and what bga takes from it
  (`edid_read_tests`, `panel_tests`), the start-up mode (`start_tests`), a
  restarted driver's killed self's frame wiped before any mode is set
  (`fresh_tests`).
- **One bring-up attempt end to end (`bring_up/`):** Memory Space on and Bus
  Master off, a function with no DISPI ID refused before a mode is written,
  a short framebuffer or register mapping refused with nothing written past
  it, a mode an adapter model clamps refused by name with nothing drawn, a
  mode past VIDEO_MEMORY_64K refused by name before a register is written,
  the firmware's mode kept when drawable with video memory wiped under it
  (`kept_tests`, `firmware_tests`), its whole DISPI state written back
  before the first unmap, at drop and on a failed attempt
  (`restore_tests`, `not_taken_tests`), never over a state unchanged and in
  QEMU's order (`write_back_tests`), and the handles giving back both
  mappings and then the claim, under the id a DGON names.
- **The display wire:** CAPS, MODES, STATE (the mode lost and set again),
  MODE_SET and FLUSH (`wire_tests`, `wire_state_tests`, `wire_lost_tests`,
  turn (`turn_tests`), the shared wire sweep (`wire_sweep/`), its place in
  the kernel's display list (`list_tests`), the service it announces the
  one Capsule.mk declares and the compositor looks up (`service_tests`),
  and the failure reasons
  (`reason_tests`).

## Run

```sh
cd userland/bga_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
