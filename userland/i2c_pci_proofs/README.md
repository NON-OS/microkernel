# i2c_pci_proofs

Host proofs for the Intel LPSS I2C controller driver
(`capsule_driver_i2c_pci`). The crate includes the shipping driver source with
`#[path]` and runs it against a register window in memory
(`nonos_devmodel`), without a boot and without a controller.

## What it proves

47 `#[test]` functions over what the DesignWare databook and Intel's LPSS
wrapper fix in writing: the LPSS function reset is released first and the
integrated DMA never is, and the PCI function gets Memory Space and never
Bus Master (`reset_tests`, `dma_tests`),
the register that proves MMIO is alive and the dead-MMIO guard
(`mmio_tests`), the input clock and the SCL count arithmetic
(`clock_tests`, `scl_tests`, `scl_count_tests`), the registers that are
read-only once the core is enabled (`enable_state_tests`), the target address
change (`target_tests`), the answer to a refused request header
(`request_refusal_tests`), the register accessor kept inside its window
(`regs_tests`), the set of addresses a client may reach (`reach_tests`) and
how it narrows on a client's read, against every head a read can bring back
(`narrow_tests`), the
rest of the LPSS wrapper bring-up (reset assert, remap address, capabilities
type) and the component checks (`lpss_tests`), standard mode and the SDA
hold register (`speed_tests`), and the device id, clock and I2Cn tables
(`table_tests`).

Transfers through the controller's FIFOs are proved in
`userland/i2c_transfer_proofs`, against `nonos_i2cmodel`.

## What it does not prove

Bus timing on a wire, and the broker half of the driver. The firmware's
touchpad record is included (`src/discover.rs`) only for the reach's size.

## Run

```sh
cd userland/i2c_pci_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
