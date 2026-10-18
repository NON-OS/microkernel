# Hardware support matrix: Platform; Display

Part of [Hardware support matrix](MATRIX.md).

## Platform

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| CPU | x86_64 cores, [SMP](../overview/glossary-rollback-index.md#smp): every core the firmware enables, up to 256 | kernel | Works | QEMU with 8 CPUs, or the host's cores when fewer, under `make boot` and CI's boot smoke; 4 under `nonos-mk-run-smp-serial-log`; 1 to 8 across the boot matrix | 0.9.2 |
| [IOMMU](../overview/glossary-hardware-broker.md#iommu) | Intel VT-d DMA remapping | kernel | Works | proof crates `kernel_proofs` (388), `mechanism_proofs` (56); QEMU `intel-iommu` | 0.9.2 |
| IOMMU | Intel VT-d interrupt remapping | kernel | Works: on in every image, and a claim on a device no unit in service remaps is refused | proof crate `kernel_proofs`; no QEMU `intel-iommu,intremap=on` boot log is recorded yet | 0.9.2 |
| IOMMU | AMD-Vi DMA and interrupt remapping | kernel | Works: on in every image, from the firmware's IVRS table, and a claim on a device no unit in service confines and remaps is refused | proof crate `kernel_proofs`; no QEMU `amd-iommu` boot log or AMD machine is recorded yet | 0.9.2 |
| [TPM](../overview/glossary-stark-proof.md#tpm) | TPM 2.0, FIFO and CRB interfaces | kernel | Works | proof crates `tpm_key_proofs` (42), `tpm_enroll_proofs` (37); QEMU `tpm-crb` with swtpm | 0.9.2 |
| ACPI | Fixed-feature power button | kernel | Partial: a press reaches the desktop as the Power key, which shows `Power off is not available from the desktop`; the desktop cannot shut down in 0.9.2 | real hardware | 0.9.2 |
| ACPI | Power button through AML (PNP0C0C) or on a hardware-reduced platform | kernel | Refused: NONOS has no AML interpreter to run it, and the desktop has no power-off either | none | 0.9.2 |
| ACPI | Battery charge, PNP0C0A | kernel | Refused: reading the charge runs AML, which NONOS cannot; the top bar shows `Battery status unavailable`, or `No battery` when the firmware declares none | none | 0.9.2 |
| GPIO | Pad level of the touchpad line: Intel Broxton family, Intel chipsets Sunrise Point to Meteor Lake, AMD (AMD0030, AMDI0030, AMDI0031, AMDI0033) | `capsule_driver_i2c_pci` with `nonos_pinctrl` | Partial: read-only, for the touchpad's interrupt line; no general GPIO driver. On other controllers the touchpad driver reads on a timer | proof crates `pinctrl_proofs` (13), `i2c_pci_proofs` (35) | 0.9.2 |
| Entropy | virtio-rng 1af4:1005, 1af4:1044 | `capsule_driver_virtio_rng` | Works | proof crate `virtio_rng_proofs` (12); QEMU | 0.9.2 |

## Display

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Display | UEFI GOP framebuffer | kernel and compositor | Works | QEMU | 0.9.2 |
| Display | virtio-gpu 1af4:1010, 1af4:1050 | `capsule_driver_virtio_gpu` | Works | proof crate `virtio_gpu_proofs` (38); QEMU `virtio-vga` | 0.9.2 |
| Display | Bochs display adapter 1234:1111 | `capsule_driver_bga` | Not supported: parked, since it re-modes the adapter and would destroy the firmware framebuffer the desktop uses | proof crate `bga_proofs` (9) | 0.9.2 |
| Display | Native GPUs: Intel 8086, AMD 1002, NVIDIA 10de, PCI class 03 | none | Not supported: no modeset driver; the desktop draws on the firmware framebuffer ([Display](../drivers/display.md)) | none | 0.9.2 |
