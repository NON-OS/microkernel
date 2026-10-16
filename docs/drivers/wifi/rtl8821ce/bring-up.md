## Bring-up

```mermaid
stateDiagram-v2
  [*] --> Claim
  Claim --> NotClaimed
  Claim --> Power
  Power --> PowerFailed
  Power --> DeadMmio
  Power --> Firmware
  Firmware --> FirmwareFailed
  Firmware --> Radio
  Radio --> NoDma
  Radio --> EfuseFailed
  Radio --> NoStationAddress
  Radio --> Ready
```

The driver goes as far as it can and then serves whatever stage it reached, so the Settings panel can show why the radio is down. Claim, Power, Firmware and Radio are the four phases.

1. Claim: `start_driver` finds the chip, claims it and maps its registers (`userland/capsule_driver_rtl8821ce/src/main.rs:87-95`, `start_driver`). With no chip the capsule exits with `EXIT_ABSENT` (2). A refused claim is retried on the shared bring-up schedule in `nonos_libc`: 7 attempts, the sleep between them starting at 100 ms and doubling up to 3.2 s (`userland/libc/src/bringup/policy.rs:30-35`, `BRINGUP_ATTEMPTS`, `BRINGUP_MAX_DELAY_MS`). Running out leaves the driver serving `NotClaimed`.
2. Power: the power-on sequence runs and the chip is read back (`userland/capsule_driver_rtl8821ce/src/main.rs:96-106`, `probe`). Failure stops at `PowerFailed` or `DeadMmio`.
3. The PCIe link is held out of L1, which gates clocks inside the chip (`userland/capsule_driver_rtl8821ce/src/main.rs:107-114`, `hold_link_awake`), and the PCIe completion timeout is switched off (`userland/capsule_driver_rtl8821ce/src/main.rs:115-121`, `disable_completion_timeout`).
4. The efuse is read on the freshly powered MAC (`userland/capsule_driver_rtl8821ce/src/main.rs:122-131`, `efuse::read`).
5. Firmware: the 8051 firmware is downloaded through reserved-page staging and DDMA (`userland/capsule_driver_rtl8821ce/src/main.rs:132-139`, `fwload::load`). Failure stops at `FirmwareFailed`.
6. The transmit and receive engines are enabled and the MAC table runs (`userland/capsule_driver_rtl8821ce/src/main.rs:140-152`, `init_trx_cfg`, `run_mac_table`). An engine that does not start also stops at `FirmwareFailed`.
7. Radio: `build_radio` maps the DMA rings, configures the PHY from the efuse and draws the station address (`userland/capsule_driver_rtl8821ce/src/serve/radio.rs:59-78`, `build_radio`). Failures stop at `NoDma`, at `EfuseFailed` for an efuse that still does not read or an RF front-end with no register table, or at `NoStationAddress` (`userland/capsule_driver_rtl8821ce/src/serve/radio.rs:128-168`, `Stage::NoStationAddress`).

A bring-up that passes every phase ends at `Ready`. The stage numbers are listed in `Stage` (`userland/capsule_driver_rtl8821ce/src/serve/stage.rs:19-43`) and their panel texts on the [Wi-Fi overview](../overview/stages.md#bring-up-stages).

The card is a Wi-Fi and Bluetooth combo. The driver hands the shared antenna to Wi-Fi and keeps the Bluetooth grant low; Bluetooth is not driven (`userland/capsule_driver_rtl8821ce/src/coex/wl_only.rs:17-34`, `take_antenna`).
