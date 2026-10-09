# i2c_pci_proofs

Host proofs for the Intel LPSS I2C controller driver
(`capsule_driver_i2c_pci`). The crate includes the shipping driver source with
`#[path]` and runs it against a register window in memory
(`nonos_devmodel`), without a boot and without a controller.

## What it proves

29 `#[test]` functions over what the DesignWare databook and Intel's LPSS
wrapper fix in writing: the LPSS reset deassert comes first (`reset_tests`),
the register that proves MMIO is alive and the dead-MMIO guard
(`mmio_tests`), the input clock and the SCL count arithmetic
(`clock_tests`, `scl_tests`, `scl_count_tests`), the registers that are
read-only once the core is enabled (`enable_state_tests`), the target address
change (`target_tests`), the answer to a refused request header
(`request_refusal_tests`), and the rest of the LPSS wrapper bring-up (reset
assert, remap address, capabilities type), the component checks, standard
mode, and the device id, clock and I2Cn tables (`lpss_tests`).

Transfers through the controller's FIFOs are proved in
`userland/i2c_transfer_proofs`, against `nonos_i2cmodel`.

## What it does not prove

Bus timing on a wire, and the broker half of the driver.

## Run

```sh
cd userland/i2c_pci_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
