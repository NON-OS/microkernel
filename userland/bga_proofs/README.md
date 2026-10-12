# bga_proofs

Host proofs for the Bochs Graphics Adapter capsule (`capsule_driver_bga`).
The crate includes the shipping DISPI, EDID, start-up and display source with
`#[path]` and runs it against a register window and video memory in memory
(`nonos_devmodel`), without a boot and without an adapter.

## What it proves

35 `#[test]` functions: the DISPI mode set's settled registers, never
GETCAPS (`mode_tests`, `mode_screen_tests`; a memory window cannot witness
the write order), the register offsets (`offset_tests`), the mode read back
(`current_tests`), the EDID read and what bga takes from it
(`edid_read_tests`, `panel_tests`), the start-up mode (`start_tests`), the
wire's CAPS, MODES, STATE, MODE_SET and FLUSH (`wire_tests`,
`wire_state_tests`, `wire_flush_tests`), its place in the kernel's display
list (`list_tests`), and the failure reasons (`reason_tests`).

The capsule itself ships in no image: it has no `Capsule.mk`, Cargo feature
or kernel mirror.

## Run

```sh
cd userland/bga_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
