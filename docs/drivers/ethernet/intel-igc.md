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

In 0.9.2 the build did not include it, the kernel mirror it names did not exist, and `net.core` did not look it up. Since then the kernel side is wired: the mirror `src/hardware/igc_capsule`, the feature `nonos-capsule-driver-igc` and its build `microkernel-driver-igc`, the broker scope row `driver.igc_0` (`src/hardware/broker/scope_table.rs`, ids in `scope_ids_nic.rs`, `IGC`), the spawn (`src/userspace/init/spawn_plan/drivers_nic.rs`, `spawn_igc`) and the card table (`src/services/registry/nics.rs`, `NICS`). Since 27 October 2026 the Standard profile carries it (`microkernel-full-gui`), started only when the inventory finds one of its ids, and the Air-Gapped image drops it (`networkFeatures`, `tools/nix/config.nix`); a release build waits on the release seal for its certificate (`scripts/baselines/unsealed.txt`). Both capsules keep their register snapshot at operation 7 (`userland/capsule_driver_igc/src/protocol/ops.rs`, `OP_STATS`). Since 27 October 2026 igc serves operation 6, `OP_RX_BATCH`, as e1000 and virtio-net do: the empty probe is answered with an empty batch, every waiting frame comes in one numbered reply of at most 24 frames and 64 KiB, and a batch asked for again by the same number is sent again rather than lost (`userland/capsule_driver_igc/src/server/handlers/rx_batch.rs`, `handle`; `src/queue/rx_batch.rs`, `take_batch`). `igc_proofs` reads its replies with `net.core`'s own `batch_served` and `batch_frames`, mounted unmodified (`src/tests/net_reader.rs`), so [the receive fault](README.md#the-receive-fault) cannot come back through it unseen. e1000e serves operation 6 the same way, read in `e1000e_proofs` by the same mounted parser (`userland/capsule_driver_e1000e/src/server/handlers/rx_batch.rs`, `handle`).

## Tests

`igc_proofs` runs the descriptor, ring, reset, semaphore, PHY and queue code against a register window in host memory, with a modelled part where a handshake needs one; its own notes say QEMU has no model of this part (`userland/igc_proofs/src/lib.rs:17-25`, `igc`). The flake check `proofs-igc_proofs` passed with 58 tests on this commit.

## See also

- [Intel Ethernet](intel.md)
- [Ethernet drivers](README.md)
- [The broker API](../broker-api.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
