## Microkernel contract

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio`,
`Dma`, and, optionally, `Debug`, as the RTL8821CE's does:

```text
CAPSULE_REQUIRED_CAPS = 0xB8038
CAPSULE_OPTIONAL_CAPS = 0x100
```

The driver reaches the card only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap` and `MkDmaMap`; it polls and binds no interrupt, so the manifest
holds no `Irq`. `Crypto` admits `CryptoRandom`, which draws the station address
and a join's nonces and SAE secrets. `Debug` is optional: a hardened build
grants none and the driver still runs. Requests arrive with `MkIpcRecvFrom` and
are answered with `MkIpcReply`. Only `net.core` and the Wi-Fi panels
(`app.settings`, `app.setup_wizard`) may send to `driver.rtl8822ce0`: the
kernel holds the endpoint to them (`src/services/registry/held.rs`).

## Interface contract

The same two request families as the RTL8821CE driver, on the one service
inbox: net_core's link protocol (`nonos_wifi_core::netif`) and the Wi-Fi
control family (`nonos_wifi_client`, tag `0x57494649`): status (4), scan (3),
connect (1), disconnect (2) and link (5). The serve loop is the RTL8821CE's own
source, mounted by path, so the layouts are byte for byte the same.

## Authority

The capsule may claim only 10ec:c822 and 10ec:c82f
(`src/hardware/broker/scope_table.rs`, `driver.rtl8822ce0`), map that
function's register BAR, and map DMA it was granted. It holds no port I/O, no
interrupt and no other device.

## Privacy and persistence

The efuse MAC is never read out: the station address is drawn from
`CryptoRandom` each boot. No log line carries an address, an SSID or a serial.
Nothing is written to storage; passphrases live only for the join that used
them.

## Runtime lifecycle

Startup finds the card, claims it, maps its BAR, powers it on with the 8822C's
own sequence, downloads `rtw8822c_fw.bin` over the shared H2C/DDMA path, reads
the efuse (RFE type, crystal, TX power, factory trims), runs the 8822C MAC
init, loads the BB, AGC, calibration-init and both RF path tables, runs the DAC
calibration, takes the antenna for Wi-Fi, draws the station address and serves.
Scan and join are the RTL8821CE's (2.4 GHz). Teardown is process exit, with the
broker revoking the device, MMIO and DMA grants.

## Failure model

Each stage that stops says so on the console and on the panel's stage byte;
the capsule keeps serving that stage instead of vanishing. A card absent from
the device list exits `EXIT_ABSENT` before claiming anything. An RFE type with
no table here stops at `EfuseFailed`. Every wait is bounded
(`rtl8822ce_proofs`).

## Current implemented surface

- Discovery and brokered claim of 10ec:c822 and 10ec:c82f.
- 8822C power sequences, efuse layout, FIFO layout, MAC init, PCIe quirks.
- Two RF paths through their direct windows; BB, AGC and RF tables; DAC
  calibration and the factory trims; channel set-up; TX power capped at 18 dBm.
- Passive scan and WPA2/WPA3 joins through the shared station, on 2.4 GHz.
- Not ported: IQK, DPK, TX gain calibration, DIG, thermal tracking, 5 GHz
  ([README.md](../README.md)).

## Wire format

Requests use the capsule header, version `1`, and the shared driver envelope.
Replies begin with a signed status word. All integers are little-endian.

## State ownership

`driver.rtl8822ce0` owns only hardware-facing state: PCI identity, grant ids,
the BAR mapping, ring metadata, the station address, radio and link state and
the installed keys. The Wi-Fi core owns scan policy, the association machine,
authentication and key derivation; `net.core` and above own addressing.

## Operating rules

- No DHCP, DNS, IP or socket policy in this capsule.
- No kernel internals, no port I/O, no inline assembly.
- No Wi-Fi profiles, passphrases or scan history persisted.
- Firmware download through bounded DMA staging with explicit completion.
- The RTL8821CE files it mounts are not edited for this chip.

## Release target

```text
driver.rtl8822ce0 -> nonos_wifi_core -> net.core -> net.ip -> apps
```

## Release evidence

Host only: `rtl8822ce_proofs` (106 tests, 95 of 95 host mutants caught,
`verification/host_mutants/rtl8822ce.json`). No hardware log exists yet.

## Release checklist

- `rtl8822ce_proofs` and `rtl8821ce_proofs` pass.
- A hardware log from an RTL8822CE machine shows the nine bring-up lines, a
  scan, a WPA2 and a WPA3 join, and a lease through `net.core`
  (`docs/hardware/real-hardware-checklist.md`).
- The owner seals `driver-rtl8822ce` (it is in `scripts/baselines/unsealed.txt`).

## Explicit non-goals today

5 GHz, beamforming, power save, hardware rate adaptation beyond the
firmware's, Bluetooth coexistence beyond giving Wi-Fi the antenna.
