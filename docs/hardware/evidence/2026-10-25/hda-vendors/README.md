# Item 6: Cirrus Logic and IDT codec setup, booted at 2b8be2d53

hw/hda-vendors 2b8be2d53 (on main 458fbae92), built as standard-dev in the
dev copy of the tree and sealed with throwaway keys; no key under `.keys/`
was read. QEMU only, under TCG. **Not hardware-verified**: QEMU has no
Cirrus, IDT, Conexant or Realtek codec, so the vendor steps ran only on
the host simulation (`hda_proofs::proofs::vendor`).

## What it does

- Cirrus CS4206 and CS4207: cs420x.c's errata sequence and DAC, ADC and
  beep coefficients; CS4208: its two. Apple machines' amplifier GPIOs.
- IDT 92HD71, 73, 83, 89 and 95: the port power map (vendor verb 7ECh), a
  port powered down only when its pin says nothing is connected.
- The HP all-GPIOs-high rule is now Realtek's alone: a Conexant HP board
  lights its mute LED with a GPIO high.
- Conexant's family-wide setup is EAPD on every pin, which every codec
  already got. Linux's `pin_amp_workaround` flag for Conexant was not
  taken: where it changes a verb could not be found in the source read,
  and nothing is written on a guess. Per-board pin fixups (any vendor) are
  not here.

Every value is cited beside it in the code (Linux 21a7f44fd076, read for
values alone, no code taken).

## Host

- hda_proofs: 212 passed (11 new in `proofs/vendor`), clippy clean; the
  capsule builds for x86_64-nonos-user with no warning.
- static-abi 39/39, `tools/nix/inputs.py --check` clean, rustfmt clean,
  every touched source file at or under 75 lines.
- Mutants: 16 of 16 killed (`MUTANTS.md`).

## Boot (`qemu.txt`, `hda.serial.log`)

`--lane iommu=intel --lane usb-tablet`: q35, VT-d, intel-hda with QEMU's
duplex codec (1af4:0022). The HDA lines are the same as main's (H-09's
after.serial.log): `[HDA] controller 8086:2668`, `codecs mask=0001`, `path
cad=0 pin=03 dac=02 hops=2`, `ready: outputs muted`, and `[AUDIO] output
driver.hda0`; no vendor step touched QEMU's codec. The desktop's first frame
let through; the only `[EXIT]` lines are the three NIC drivers with no card.
Requester ids are replaced by `<bdf>` (K10).
