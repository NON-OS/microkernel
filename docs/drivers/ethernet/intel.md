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
