## Reading the status reply

The status reply carries the stage byte the panel shows, then the step bring-up stopped at, a detail word, CSR_HW_REV, CSR_HW_RF_ID and the scan counters (`userland/capsule_driver_iwlwifi/src/server/runner.rs:103-114`, `view`). Steps and details are set by `Failure` (`userland/capsule_driver_iwlwifi/src/firmware/gen3/outcome.rs:120-158`, `detail`).

| Step | Meaning | Stage | Detail |
|---|---|---|---|
| 1 | register window too small | `DeadMmio` | 0 |
| 2 | CSR_HW_REV reads all ones | `DeadMmio` | 0 |
| 3 | NIC not ready, MAC clock not ready, no NIC access | `PowerFailed` | 1, 2, 3 |
| 4 | no bundled firmware | `NoAirPath` | 0x1xxxx PCI id, 0x2xxxx MAC type, 0x3xxxx RF type, 0x40000 dual radio, 0x5000s MA step, 0x6000i image |
| 5 | bundled image did not parse | `FirmwareFailed` | 0 |
| 6 | the broker refused a DMA region | `NoDma` | 0 |
| 7 | boot to ALIVE failed | `FirmwareFailed`, or `PowerFailed` for 0x11 to 0x13 and 0x70 | 0x11 to 0x13 start, 0x21 to 0x23 layout, 0x30 no ALIVE, 0x41 to 0x43 no ALIVE notice, 0x5ssss ALIVE status, 0x61 to 0x63 PNVM, 0x70 |
| 8 | a post-ALIVE command failed | `FirmwareFailed` | 0x01ggcc0w for a command (group, command, wait result), 0x0200000w for INIT_COMPLETE, 0x03000000 NVM, 0x04000000 MCC, 0x05000000 unsupported API |
| 9 | the firmware raised its error cause while running | `FirmwareFailed` | 0 |
| 10 | the device's Bus Master did not read back stopped, or the broker refused to turn it on | `NoDma` | 1 not stopped, 2 refused |
| 11 | a firmware command went unanswered | `FirmwareFailed` | 0 |
| 12 | a transmitted frame went unanswered for 2.5 s | `FirmwareFailed` | 0 |

In 0x6000i, the image number is 0 for so-a0-gf-a0, 1 for so-a0-hr-b0, 2 for ty-a0-gf-a0 and 3 for ma-b0-gf-a0 (`userland/capsule_driver_iwlwifi/src/firmware/gen3/select.rs:32-44`, `Image`).

## The older driver protocol

The driver's own NIWF protocol and the legacy firmware load it carried are deleted; a request that is neither the Wi-Fi control family nor net_core's link protocol gets no reply (`userland/capsule_driver_iwlwifi/src/server/control/route.rs`, `Route::Other`).

## Firmware

The capsule links two files from `nonos-bootloader/firmware/intel/` with `include_bytes!`, so-a0-gf-a0-86 and so-a0-hr-b0-84, and boots them (`userland/capsule_driver_iwlwifi/src/firmware/blob.rs`, `gen3_blob`). They are Intel binaries from linux-firmware, under [the Intel licence](../../../../nonos-bootloader/firmware/intel/LICENSE), not AGPL code. Adding the TY or MA image to the tree and to `gen3_blob` is what those two cards lack.

## Tests

`iwlwifi_proofs` compiles the driver's source with `#[path]` and runs the SO boot to ALIVE, the passive scan, and WPA2 and WPA3-SAE joins with data both ways against `gen3_model` and a scripted access point. The flake check `proofs-iwlwifi_proofs` passed with 205 tests on this commit. These tests do not show that a real card's firmware accepts the commands.

```sh
cd userland/iwlwifi_proofs && cargo test --release --config profile.release.overflow-checks=true
```

Not tested in this release.
