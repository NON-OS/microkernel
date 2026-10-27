# capsule_driver_iwlwifi

## Role

`capsule_driver_iwlwifi` is the Intel Wi-Fi PCIe hardware capsule. It owns the
Intel wireless device claim, BAR0 mapping, interrupt grant, DMA grants,
firmware selection and load, and, on the SO platforms (AX211, and AX201 on an
SO platform), the discrete AX210 (8086:2725), Meteor Lake (8086:7E40, 2729)
and the Wi-Fi 7 BE200 (8086:272B) and BE201 (8086:A840, 7740, 4D40), the
firmware bring-up, passive scanning, joining WPA2 and WPA3
networks and carrying their data for `driver.iwlwifi0`.

```text
Settings / setup / net_core (nonos_wifi_client)
        |   Wi-Fi control family: status, scan, connect, disconnect, link
        v
driver.iwlwifi0 -- brokered MMIO/IRQ/DMA --> Intel Wi-Fi PCIe device
        |   the MLME, SAE, supplicant, CCMP and station checks of
        |   nonos_wifi_core (shared with driver.rtl8821ce0)
        ^
net_core -- link protocol (NNET): link status, MAC, Ethernet frames
```

The capsule is not an IP stack, socket layer, DHCP client, or network profile
manager. Those layers stay above the hardware driver.

## Parts

- [The microkernel contract](docs/microkernel.md)
- [The interface contract](docs/interface.md)
- [Firmware, authority, privacy and persistence](docs/firmware.md)
- [The runtime lifecycle: setup, bring-up, the serving loop](docs/lifecycle.md)
- [The runtime lifecycle: joins, the open port, interrupts](docs/joins.md)
- [The failure model and the implemented surface](docs/failures.md)
- [The wire, state, operating rules and release](docs/release.md)
- [Real hardware bring-up: setup to scanning](docs/hardware.md)
- [Real hardware bring-up: joins](docs/hardware_joins.md)

## Explicit non-goals today

This slice does not implement open networks, hidden networks, HT, VHT or HE
rates, rate scaling (data goes at the highest basic rate), QoS (WMM),
aggregation, power save beyond Linux's balanced scheme, active scanning,
6 GHz channels, regulatory updates beyond the world domain,
interrupt-driven operation, monitor mode, AP mode, or roaming. The 7265, 8265,
9260/9560 and AX200 parts, and AX201 outside the SO platforms, do not get a
radio from this driver yet.

The discrete AX210 and Meteor Lake run linux-firmware's
`iwlwifi-ty-a0-gf-a0-86.ucode` and `iwlwifi-ma-b0-gf-a0-86.ucode` (or
`-gf4-a0-86` for a dual-radio module) with their `.pnvm` files; the BE200 and
BE201 run `iwlwifi-gl-c0-fm-c0-92.ucode` and `iwlwifi-bz-b0-fm-c0-92.ucode`
with theirs, in the BZ family's second CSR layout. Every file is
linux-firmware's at commit bf992df7 (licence: Redistributable,
LICENCE.iwlwifi_firmware / `nonos-bootloader/firmware/intel/LICENSE`). Those
paths are host-proven, never booted on those parts ([firmware](docs/firmware.md),
and the test plans in `docs/hardware/real-hardware-checklist.md`).

## Verification

- Capsule: `(cd userland/capsule_driver_iwlwifi && cargo build --release
  --target ../x86_64-nonos-user.json -Zbuild-std=core,alloc
  -Zbuild-std-features=compiler-builtins-mem)`
- Proofs: `(cd userland/iwlwifi_proofs && cargo test --release)`,
  `(cd userland/nonos_wifi_core_proofs && cargo test --release)` and
  `(cd userland/wifi_panel_proofs && cargo test --release)`
- Build: `make -B nonos-mk-driver-iwlwifi`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-iwlwifi`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [network stack](../../docs/handbook/network/stack.md).
