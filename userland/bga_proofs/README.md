# bga_proofs

Host proofs for the parked Bochs Graphics Adapter capsule
(`capsule_driver_bga`). The crate includes the shipping constants, error and
register source with `#[path]` and runs the DISPI mode set against a register
window in memory (`nonos_devmodel`), without a boot and without an adapter.

## What it proves

23 `#[test]` functions: the DISPI mode set leaves the resolution asked for, 32
bits per pixel and the linear framebuffer decoding in the registers
(`mode_tests`; a memory window cannot witness the write order, only the
settled state), the register offsets (`offset_tests`), the framebuffer clear
(`clear_tests`), a mode against the mapping it must fit at every bound
(`span_tests`), the DISPI ID accepted only from 0xB0C0 to 0xB0C5 and a mode
that does not read back as set refused (`check_tests`; a memory window
always reads back what was written, so the read-back is held against windows
holding another mode), and the failure reasons (`reason_tests`). One
bring-up attempt runs end to end against `libc_shim`'s broker
(`setup_tests`, `setup_bound_tests`, `setup_claim_tests`,
`setup_confirm_tests`): the PCI command left with Memory Space on and Bus
Master off, a function with no DISPI ID refused before a mode is written, a
framebuffer or register mapping shorter than the mode needs refused before
the mode is set with nothing written past it, a mode an adapter model
clamps as it is set refused by name with nothing drawn, and the handles
giving back both mappings and then the claim.

The capsule itself ships in no image: it has no `Capsule.mk`, Cargo feature
or kernel mirror.

## Run

```sh
cd userland/bga_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
