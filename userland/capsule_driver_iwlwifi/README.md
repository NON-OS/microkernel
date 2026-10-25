# capsule_driver_iwlwifi

## Role

`capsule_driver_iwlwifi` is the Intel Wi-Fi PCIe hardware capsule. It owns the
Intel wireless device claim, BAR0 mapping, interrupt grant, DMA grants,
firmware selection and load, and, on the SO platforms (AX211, and AX201 on an
SO platform), the firmware bring-up, passive scanning, joining WPA2 and WPA3
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
aggregation, power save, beacon-loss detection, active scanning, 6 GHz
channels, regulatory updates beyond the world domain,
interrupt-driven operation, monitor mode, AP mode, or roaming. The 7265, 8265,
9260/9560 and AX200 parts, and AX201 outside the SO platforms, do not get a
radio from this driver yet; the discrete AX210 and Meteor Lake lack only their
firmware files.

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
