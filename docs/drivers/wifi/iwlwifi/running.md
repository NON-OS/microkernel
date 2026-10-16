## Station address

Each boot draws a locally administered address from kernel randomness; the card's factory address is never used (`userland/capsule_driver_iwlwifi/src/server/radio/bring.rs:184-190`, `draw`). With no randomness the interface takes the fixed 02:00:00:00:00:01, only the passive scan runs and nothing is transmitted (`userland/capsule_driver_iwlwifi/src/firmware/gen3/up.rs:42`, `SCAN_IF_ADDR`). A join needs the radio up, a drawn address and the right command layouts (`userland/capsule_driver_iwlwifi/src/server/radio/join.rs:99-101`, `can_join`); otherwise connect, disconnect and link are answered with -38 (`userland/capsule_driver_iwlwifi/src/server/control.rs:94-109`, `route`).

## Scanning

- The scan is passive: the request sets the firmware's forced-passive flag with a 110 ms dwell (`userland/capsule_driver_iwlwifi/src/firmware/gen3/scan.rs:38-41`, `GEN_FLAGS_FORCE_PASSIVE`, `DWELL_PASSIVE`). No probe request is sent.
- The channels are the 2.4 and 5 GHz entries of the NVM; 6 GHz is not scanned (`userland/capsule_driver_iwlwifi/src/firmware/gen3/nvm.rs:38-40`, `NVM_CHANNELS`).
- A sweep may run 150 ms per channel plus 2 s before it counts as stalled (`userland/capsule_driver_iwlwifi/src/firmware/gen3/sweep.rs:42-50`, `budget_ms`), and the next starts 3 s after one ends (`userland/capsule_driver_iwlwifi/src/server/radio/mod.rs:54-55`, `REST_MS`).
- While a scan runs, the serving loop wakes every 50 ms to pump it (`userland/capsule_driver_iwlwifi/src/server/runner.rs:25-27`, `SCAN_TICK_MS`).

## Joining

- A network saved as hidden is refused with -2, because this driver sends no probe and a hidden network's beacon carries no name (`userland/capsule_driver_iwlwifi/src/server/radio/join.rs:122-126`, `CODE_NOT_FOUND`).
- The hunt for the network's beacon listens only (`userland/capsule_driver_iwlwifi/src/firmware/gen3/join/hunt.rs:21-30`, `parse_beacon`).
- The policy is WPA3-SAE when offered, WPA2 otherwise, and only SAE for a network saved as WPA3 (`userland/capsule_driver_iwlwifi/src/server/radio/join.rs:163`, `JoinPolicy::WPA3_ONLY`). Unlike the RTL8821CE, this driver does not retry a failed WPA3 join with WPA2.
- The firmware is asked for 900 ms of time on the channel for the join (`userland/capsule_driver_iwlwifi/src/firmware/gen3/station/session.rs:43`, `JOIN_SESSION_MS`).
- An unanswered frame is resent every 300 ms, at most 6 times in a row, and the whole exchange gets 7.5 s (`userland/capsule_driver_iwlwifi/src/firmware/gen3/join/exchange/limits.rs:11-18`, `RETX_MS`, `IDLE_TRIES`, `EXCHANGE_MS`).
- Management and EAPOL frames go at the lowest basic rate and data at the highest basic rate; no rate scaling runs (`userland/capsule_driver_iwlwifi/src/firmware/gen3/station/rates.rs:28-32`, `IWL_TX_FLAGS_CMD_RATE`).

A join returns the RTL8821CE's status codes, plus -3 and -4 when the pairwise or group key does not go into the card (`userland/capsule_driver_iwlwifi/src/server/join_wire.rs:54-75`, `CODE_GROUP_KEY`); the panel texts are on the [Wi-Fi overview](../README.md#what-a-join-answers). A deauthentication or disassociation from the access point ends the link (`userland/capsule_driver_iwlwifi/src/firmware/gen3/join/link.rs:34-44`, `parse_leave`). Read from the code, nothing watches for lost beacons, so a link whose access point goes silent stays up until a disconnect.

## Authority

The [manifest](../../../overview/glossary.md#manifest) asks for the [capability](../../../overview/glossary.md#capability-word) mask 0xF8038: IPC, Memory, Crypto, Driver, DeviceEnum, Mmio, Irq and Dma (`userland/capsule_driver_iwlwifi/Capsule.mk:15-17`, `CAPSULE_REQUIRED_CAPS`). The service is `driver.iwlwifi0` on port 4228 (`userland/capsule_driver_iwlwifi/Capsule.mk:13`, `CAPSULE_SERVICE_ENDPOINT`). The kernel's spawn request holds no Debug in any build (`src/hardware/iwlwifi_capsule/spawn.rs:50-59`, `requested_caps`), so the driver's own console lines never reach the log. The Settings panel's stage line and the status reply are the record.
