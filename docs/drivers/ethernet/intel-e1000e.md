# Intel e1000e (82574, 82583, I217, I218, I219)

The e1000e [capsule](../../overview/glossary-boot-root-record.md#capsule) is written and host-tested but built into no image in 0.9.2. The e1000 driver in the image is on the [Intel Ethernet](intel.md) page.

## Device ids

The id table follows the boards Linux binds them to (`userland/capsule_driver_e1000e/src/constants/ids.rs:21-51`, `PCH_PTP`). Vendor 8086:

| Board | Chips | Device ids |
|---|---|---|
| 82574 | 82574L, 82574LA | 10d3, 10f6 |
| 82583 | 82583V | 150c |
| pch_lpt | I217-LM, I217-V, I218-LM, I218-V and their second and third variants | 153a, 153b, 155a, 1559, 15a0, 15a1, 15a2, 15a3 |
| pch_spt | I219-LM, I219-V, variants 2 to 5 and 12 | 156f, 1570, 15b7, 15b8, 15b9, 15d7, 15d8, 15e3, 15d6, 0d53, 0d55 |
| pch_cnp | I219 variants 6 to 11 | 15bd, 15be, 15bb, 15bc, 15df, 15e0, 15e1, 15e2, 0d4e, 0d4f, 0d4c, 0d4d |
| pch_tgp | I219 variants 13 to 15 | 15fb, 15fc, 15f9, 15fa, 15f4, 15f5 |
| pch_adp | I219 variants 16, 17, 19, 22, 23 | 0dc5, 0dc6, 1a1e, 1a1f, 1a1c, 1a1d, 0dc7, 0dc8, 550c, 550d |
| pch_mtp | I219 variants 18, 20, 21, 24 | 550a, 550b, 550e, 550f, 5510, 5511, 57a0, 57a1 |
| pch_ptp | I219 variants 25, 27, 29 | 57b3, 57b4, 57b7, 57b8, 57b9, 57ba |

Each list maps to a family that selects the bring-up steps for it (`userland/capsule_driver_e1000e/src/constants/family.rs:37-45`, `Family::I82574`).

## What the code does

The capsule has the same link protocol and the same polled, interrupt-free design as e1000, with separate receive and transmit rings, and draws its station address the same way (`userland/capsule_driver_e1000e/src/init/station_address.rs:22-26`, `draw`). Its mask is 0xB8038 with Debug optional (`userland/capsule_driver_e1000e/Capsule.mk:26-30`, `CAPSULE_REQUIRED_CAPS`).

## Why it is not in 0.9.2

- The build does not include its makefile, whose `CAPSULE_SLUG` is `driver-e1000e`; a comment in it says wiring it into the kernel and the stack is a separate step (`userland/capsule_driver_e1000e/Capsule.mk:8-11`, `CAPSULE_SLUG`). The Ethernet makefiles the build includes are those of virtio-net, e1000, RTL8139 and RTL8169 (`mk/20-build.mk:540-550`, `capsule_driver_e1000`).
- The [kernel mirror](../../overview/glossary-hardware-broker.md#kernel-mirror) it names, `src/hardware/e1000e_capsule`, does not exist in the tree (`userland/capsule_driver_e1000e/Capsule.mk:31`, `CAPSULE_KERNEL_MIRROR`), so the kernel cannot start it.
- The kernel holds no endpoint for it, and `net.core` does not look it up (`userland/capsule_net_core/src/setup/candidates.rs:25-38`, `WIRED_NICS`).

## Tests

`e1000e_proofs` runs the driver's files against host memory and a modelled register window. The flake check `proofs-e1000e_proofs` passed with 48 tests on this commit. No QEMU run and no hardware boot exist for it.

## See also

- [Intel Ethernet](intel.md)
- [Ethernet drivers](README.md)
- [The broker API](../broker-api.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
