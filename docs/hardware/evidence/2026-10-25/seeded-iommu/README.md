# Item 7: DMAR, IVRS and `_DSM` in seeded sweeps, booted at a22438a79

hw/seeded-iommu a22438a79 (on main 1beacdd37), built as standard-dev in
the dev copy of the tree and sealed with throwaway keys; no key under
`.keys/` was read. QEMU only, under TCG. **Not hardware-verified.**

## Host

- kernel_proofs, whole crate: 1690 passed; the seeded sweep 23 tests, two
  new (DMAR, IVRS), 3000 seeds each. Clippy: the crate's only warnings are
  main's own, in `setup_outcome` (kept_table.rs, outcome_rule.rs).
- acpi_aml_proofs: 93 passed, one new sweep (20000 touchpads), clippy clean.
- x86_64 default and nonos-smp 0 warnings; aarch64 main's 5.
- static-abi 39/39, `tools/nix/inputs.py --check` clean, rustfmt clean, every
  touched source file at or under 75 lines.
- Mutants: `MUTANTS.md`.

## The defect the IVRS sweep found

`ivhd_kind.rs` kept a range END (04h) below its START as a span. It covers
no id, but it takes one of MAX_SPANS' 64 slots, so firmware with junk
ranges could push real spans out. It is now dropped, as `ivhd_alias.rs`
drops the inverted alias (mutant S1).

## Boot (`qemu-amd-vi.txt`, `amd-vi.serial.log`)

`--lane iommu=amd --lane usb-tablet`: q35 with QEMU's AMD IOMMU, so the
kernel reads QEMU's IVRS through the changed walk. `[AMD-VI] IVRS
present`, `IOMMU translation enabled, units=1`, every other requester id
blocked; `[IOMMU] amd-vi present, enforcing=1`; the desktop's first frame
let through. The only `[EXIT]` lines are the three NIC drivers with no card.
QEMU's IVRS has no inverted range, so the boot shows the walk unchanged on
a sound table; the inverted case is host-tested only. Requester ids are
replaced by `<bdf>` (K10).
