## Verification

There is no QEMU model of this chip and it has not run on hardware. What is
proven is proven on the host, against the real source:

- Capsule: `(cd userland/capsule_driver_rtl8822ce && cargo build --release
 --target ../x86_64-nonos-user.json -Zbuild-std=core,alloc
 -Zbuild-std-features=compiler-builtins-mem)`, no warnings.
- Proofs: `(cd userland/rtl8822ce_proofs && cargo test)`. The crate mounts
  this driver's own modules by `#[path]` over a modelled register file
  (`src/model.rs`) and checks each against rtw88: the ids and discovery,
  whole tagged log lines, the power tables, efuse unpacking and its bounds,
  the efuse facts, the firmware image and header, the FIFO page layout and
  MAC table, the PCIe quirks, RF access on both paths, the PHY tables and
  their order, the set-up order, the channel, TX power and its cap, the
  trims, the DAC calibration and its bounds, the PHY status signal, the
  coexistence grant and the start-up refusals.
- Mutants: `tools/nonos-host-mutants verification/host_mutants/rtl8822ce.json`
  removes one rule or bound at a time from the driver; each must make a
  proof fail.
- The RTL8821CE's own proofs must still pass in full:
  `(cd userland/rtl8821ce_proofs && cargo test)`.
- Tables: `tools/nonos-rtw88-tables --check <rtw8822c_table.c>` says
  whether the five table files are what the pinned rtw88 file gives.
