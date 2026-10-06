# Intel Ethernet

NONOS has three Intel Ethernet driver [capsules](../../overview/glossary.md#capsule): e1000 for the 8254x family is in the image, while e1000e (82574, 82583, I217, I218, I219) and igc (I225, I226) are written and host-tested but built into no image in 0.9.2.

## Which driver takes which chip

| Driver | Chips | In the 0.9.2 image | State |
|---|---|---|---|
| `driver.e1000_0` | 8254x family, 28 device ids | yes | Partial: host tests pass, [receive fault](README.md#the-receive-fault), no hardware report, no QEMU run target |
| `driver.e1000e_0` | 82574, 82583, and the I217, I218, I219 PHYs on Intel PCH chipsets, 64 device ids | no | Not supported: not built, not started, not bound by `net.core` |
| `driver.igc_0` | I225, I226 and their variants, 16 device ids | no | Not supported: not built, not started, not bound by `net.core` |

An Intel I217, I218, I219, I225 or I226 port has no driver in the 0.9.2 image. The e1000 driver that is in the image carries [the receive fault](README.md#the-receive-fault); read [what to use instead](../wifi/not-supported.md#what-to-use-instead) before you plan on a wired link.

## e1000 (8254x)

### Device ids

The driver takes an Intel PCI function of class network, subclass Ethernet, whose device id is in `E1000_DEVICE_IDS` and whose BAR0 is a memory BAR (`userland/capsule_driver_e1000/src/constants/pci.rs:19-23`, `E1000_DEVICE_IDS`; `userland/capsule_driver_e1000/src/discover.rs:59-65`, `is_match`). The ids, vendor 8086:

100e, 100f, 1010, 1011, 1012, 1013, 1014, 1015, 1016, 1017, 1018, 1019, 101a, 101d, 101e, 1026, 1027, 1028, 1075, 1076, 1077, 1078, 1079, 107a, 107b, 107c, 1099, 10b5.

### How it runs

- The service is `driver.e1000_0` on port 4210 (`userland/capsule_driver_e1000/Capsule.mk:16`, `CAPSULE_SERVICE_ENDPOINT`).
- The [capability](../../overview/glossary.md#capability-word) mask is 0xB8038: IPC, Memory, Crypto, Driver, DeviceEnum, Mmio and Dma, with no optional Debug (`userland/capsule_driver_e1000/Capsule.mk:23`, `CAPSULE_REQUIRED_CAPS`). The kernel's spawn request is the same set (`src/hardware/e1000_capsule/spawn.rs:56-65`, `requested_caps`), so the driver's own log lines are dropped in every build.
- It binds no interrupt line and never sets IMS (`userland/capsule_driver_e1000/src/setup/sequence.rs:24-27`, `IMS`); the reset masks every cause (`userland/capsule_driver_e1000/src/init/reset.rs:55`, `REG_IMC`).
- It draws a new station address every boot and never uses the EEPROM address; without randomness it does not come up (`userland/capsule_driver_e1000/src/init/station_address.rs:22-31`, `draw`).
- It answers health check (1), link status (2), MAC address (3), transmit (4), receive (5) and a register snapshot (6) (`userland/capsule_driver_e1000/src/protocol/ops.rs:23-28`, `OP_STATS`). `net.core` reads operation 6 as a receive batch, which is the cause of [the receive fault](README.md#the-receive-fault).

The kernel starts it on every boot of the full image; on a machine without one of these chips it exits with code 2 and claims nothing. See the [Ethernet overview](README.md#behaviour-every-pci-driver-shares).

### Tests

`e1000_proofs` drives the real receive and transmit rings with hostile descriptor values and checks that no copy leaves its slot, and runs the reset and bring-up steps against a modelled part. The flake check `proofs-e1000_proofs` passed with 20 tests on this commit. Kani harnesses sit in `userland/e1000_proofs/src/kani_proofs.rs`; the flake check does not run them.

The `nonos-mk-ethernet-prod` profile builds the desktop with this driver, but no QEMU run target attaches an e1000 device, so this release has no QEMU run for it.

## e1000e (82574, 82583, I217, I218, I219)

### Device ids

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

### What the code does

The capsule has the same link protocol and the same polled, interrupt-free design as e1000, with separate receive and transmit rings, and draws its station address the same way (`userland/capsule_driver_e1000e/src/init/station_address.rs:22-26`, `draw`). Its mask is 0xB8038 with Debug optional (`userland/capsule_driver_e1000e/Capsule.mk:26-30`, `CAPSULE_REQUIRED_CAPS`).

### Why it is not in 0.9.2

- The build does not include its makefile, whose `CAPSULE_SLUG` is `driver-e1000e`; a comment in it says wiring it into the kernel and the stack is a separate step (`userland/capsule_driver_e1000e/Capsule.mk:8-11`, `CAPSULE_SLUG`). The Ethernet makefiles the build includes are those of virtio-net, e1000, RTL8139 and RTL8169 (`mk/20-build.mk:532-542`, `capsule_driver_e1000`).
- The kernel mirror it names, `src/hardware/e1000e_capsule`, does not exist in the tree (`userland/capsule_driver_e1000e/Capsule.mk:31`, `CAPSULE_KERNEL_MIRROR`), so the kernel cannot start it.
- The kernel holds no endpoint for it, and `net.core` does not look it up (`userland/capsule_net_core/src/setup/candidates.rs:25-38`, `WIRED_NICS`).

### Tests

`e1000e_proofs` runs the driver's files against host memory and a modelled register window. The flake check `proofs-e1000e_proofs` passed with 48 tests on this commit. No QEMU run and no hardware boot exist for it.

## igc (I225, I226)

### Device ids

The table holds every I225 and I226 id Linux's igc driver binds, including the blank-NVM ones (`userland/capsule_driver_igc/src/constants/pci.rs:24-41`, `IGC_DEVICE_IDS`). Vendor 8086:

| Device id | Part |
|---|---|
| 15f2 | I225-LM |
| 15f3 | I225-V |
| 15f8 | I225-I |
| 15f7 | I220-V |
| 3100 | I225-K |
| 3101 | I225-K2 |
| 3102 | I226-K |
| 5502 | I225-LMVP |
| 5503 | I226-LMVP |
| 0d9f | I225-IT |
| 125b | I226-LM |
| 125c | I226-V |
| 125d | I226-IT |
| 125e | I221-V |
| 125f | I226 with a blank NVM |
| 15fd | I225 with a blank NVM |

### What the code does

One advanced receive queue and one advanced transmit queue, polled, with the station address drawn every boot and never read from the NVM (`userland/capsule_driver_igc/src/init/station_address.rs:24-28`, `draw`). The service would be `driver.igc_0` on port 4272 (`userland/capsule_driver_igc/Capsule.mk:15`, `CAPSULE_SERVICE_ENDPOINT`). The driver takes the first matching function in the [hardware broker](../../overview/glossary.md#hardware-broker)'s device list and no other (`userland/capsule_driver_igc/src/discover/scan.rs:27-55`, `find_igc`).

The tree has a file `nonos-bootloader/firmware/intel/i225-ethernet.bin` of 51 bytes; no code reads it, and the igc driver needs no firmware file.

### Why it is not in 0.9.2

The same three reasons as e1000e: the build does not include it, the kernel mirror it names does not exist (`userland/capsule_driver_igc/Capsule.mk:29`, `CAPSULE_KERNEL_MIRROR`), and `net.core` does not look it up. Both capsules also answer operation 6 with a register snapshot and status 0, as e1000 does, so wiring either in as it stands would bring [the receive fault](README.md#the-receive-fault) with it (`userland/capsule_driver_igc/src/server/runner.rs:71`, `OP_STATS`).

### Tests

`igc_proofs` runs the descriptor, ring, reset, semaphore, PHY and queue code against a register window in host memory, with a modelled part where a handshake needs one; its own notes say QEMU has no model of this part (`userland/igc_proofs/src/lib.rs:17-25`, `igc`). The flake check `proofs-igc_proofs` passed with 58 tests on this commit.

## To run the proofs

The flake runs each [proof crate](../../overview/glossary.md#proof-crate) with overflow checks on. The same run by hand:

```sh
cd userland/e1000_proofs && cargo test --release --config profile.release.overflow-checks=true
```

Not tested in this release.

## See also

- [Ethernet drivers](README.md)
- [Realtek Ethernet](realtek.md)
- [USB networking](usb-net.md)
- [The broker API](../broker-api.md)
- [Writing a driver](../writing-a-driver.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
