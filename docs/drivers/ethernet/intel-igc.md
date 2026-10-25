# Intel igc (I225, I226)

The igc [capsule](../../overview/glossary-boot-root-record.md#capsule) is written and host-tested but built into no image in 0.9.2. The e1000 driver in the image is on the [Intel Ethernet](intel.md) page.

## Device ids

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

## What the code does

One advanced receive queue and one advanced transmit queue, polled, with the station address drawn every boot and never read from the NVM (`userland/capsule_driver_igc/src/init/station_address.rs:24-28`, `draw`). The service would be `driver.igc_0` on port 4272 (`userland/capsule_driver_igc/Capsule.mk:15`, `CAPSULE_SERVICE_ENDPOINT`). The driver takes the first matching function in the [hardware broker](../../overview/glossary-hardware-broker.md#hardware-broker)'s device list and no other (`userland/capsule_driver_igc/src/discover/scan.rs:27-55`, `find_igc`).

The tree has a file `nonos-bootloader/firmware/intel/i225-ethernet.bin` of 51 bytes; no code reads it, and the igc driver needs no firmware file.

## Why it is not in 0.9.2

In 0.9.2 the build did not include it, the kernel mirror it names did not exist, and `net.core` did not look it up. Since then the kernel side is wired: the mirror `src/hardware/igc_capsule`, the feature `nonos-capsule-driver-igc` and its build `microkernel-driver-igc`, the broker scope row `driver.igc_0` (`src/hardware/broker/scope_table.rs`, ids in `scope_ids_nic.rs`, `IGC`), the spawn (`src/userspace/init/spawn_plan/drivers_nic.rs`, `spawn_igc`) and the card table (`src/services/registry/nics.rs`, `NICS`). It has no owner certificate, so no image carries it until the owner signs it. Both capsules keep their register snapshot at operation 7 (`userland/capsule_driver_igc/src/protocol/ops.rs:30`, `OP_STATS`) and do not serve operation 6; `net.core` takes any answer but an empty batch as a driver without batches and reads frames with operation 5, so wiring either in would not bring back [the receive fault](README.md#the-receive-fault) (`userland/capsule_net_core/src/device/batch_served.rs:29-31`, `served`).

## Tests

`igc_proofs` runs the descriptor, ring, reset, semaphore, PHY and queue code against a register window in host memory, with a modelled part where a handshake needs one; its own notes say QEMU has no model of this part (`userland/igc_proofs/src/lib.rs:17-25`, `igc`). The flake check `proofs-igc_proofs` passed with 58 tests on this commit.

## See also

- [Intel Ethernet](intel.md)
- [Ethernet drivers](README.md)
- [The broker API](../broker-api.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
