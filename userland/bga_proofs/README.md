# bga_proofs

Host proofs for the parked Bochs Graphics Adapter capsule
(`capsule_driver_bga`). The crate includes the shipping constants, error and
register source with `#[path]` and runs the DISPI mode set against a register
window in memory (`nonos_devmodel`), without a boot and without an adapter.

## What it proves

9 `#[test]` functions: the DISPI mode set leaves the resolution asked for, 32
bits per pixel and the linear framebuffer decoding in the registers
(`mode_tests`; a memory window cannot witness the write order, only the
settled state), the register offsets (`offset_tests`), the framebuffer clear
(`clear_tests`), and the failure reasons (`reason_tests`).

The capsule itself ships in no image: it has no `Capsule.mk`, Cargo feature
or kernel mirror.

## Run

```sh
cd userland/bga_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
