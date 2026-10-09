# capsule_driver_rtl8821ce

## Role

`capsule_driver_rtl8821ce` is the Realtek RTL8821CE PCIe Wi-Fi hardware capsule.
It owns the Realtek wireless device claim, BAR mapping, DMA rings, firmware
download, the MAC/PHY/RF bring-up, scanning and joining for
`driver.rtl8821ce0`.

```text
net.core, Settings, setup wizard (the only senders the kernel admits)
 |
 v
driver.rtl8821ce0 -- brokered MMIO/DMA, polled --> Realtek RTL8821CE PCIe device
```

The 802.11 management, the WPA3-SAE exchange (group 19, hash-to-element and
hunting-and-pecking), the WPA2 four-way and group key handshakes, key
derivation, CCMP and the receive checks run in the shared `nonos_wifi_core`
crate; IP, DHCP, DNS, and sockets stay in the upper network capsules. This
capsule drives the radio and moves frames; it does not own network profiles or
credentials.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio`,
`Dma`, and, optionally, `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0xB8038
CAPSULE_OPTIONAL_CAPS = 0x100
```

`Debug` (0x100) is optional so a serial-debug kernel can report radio
bring-up progress; a hardened build grants no `Debug` and the driver
still spawns. The driver reaches hardware only through `MkDeviceList`,
`MkDeviceClaim`, `MkMmioMap`, and `MkDmaMap`; it polls and binds no interrupt,
so the manifest holds no `Irq`. `Crypto` admits `CryptoRandom`, which draws
the station address and a join's nonces and SAE secrets. The kernel validates
the signed manifest, routes IPC, brokers grants, and revokes every grant on
capsule exit.

Only `net.core` and the Wi-Fi panels (`app.settings` in any of its three
windows, and `app.setup_wizard`) may send to `driver.rtl8821ce0`: the kernel
holds the endpoint to them (`src/services/registry/held.rs`), by name and by
pid.

## Interface contract

Two request families arrive on the one service inbox: net_core's link
protocol (`nonos_wifi_core::netif`: link status, transmit, receive) and the
Wi-Fi control family (`nonos_wifi_client`, tag `0x57494649`):

| Operation | Input | Output |
|---|---|---|
| status (4) | none | bring-up stage, data-path counters, efuse and BAR diagnostics, receive refusals, rekeys |
| scan (3) | none | passes, raw frames, beacons, then the network list (signal, WPA2/WPA3 flags, SSID) |
| connect (1) | SSID, passphrase, optional flags octet (WPA3-only, hidden) | status code, handshake counters, state, AKM, AP status or reason code |
| disconnect (2) | none | teardown result |
| link (5) | none | associated, BSSID, SSID, AKM |

Unknown operations reply with an error status word, and a frame neither family
takes is answered rather than left without a reply. Every length in a connect
body is checked before use; the flags octet is optional, so an older client
joins as before.

## Firmware

The capsule downloads the Realtek RTL8821CE firmware image into the device over
the H2C/DDMA path. Firmware bytes are linked into the capsule with
`include_bytes!`, so no filesystem authority is required at boot. The firmware
path validates the header, stages the sections into a brokered DMA window, drives
the download registers, and waits for the firmware-ready state before MAC init.

## Authority

The capsule may enumerate PCI devices, claim one supported Realtek Wi-Fi
function, map its BARs, and allocate broker-owned DMA rings. It binds no
interrupt: discovery does not look at the INTx line and the rings are polled.
It has no filesystem authority, no credential authority, and no network-stack
authority. Passphrases arrive only for the duration of a connect request and are
handed to the shared supplicant; the driver keeps no profile store.

## Privacy and persistence

The capsule stores no SSIDs, passphrases, scan history, peer MAC history, DHCP
leases, or IP state. Runtime state is limited to grant ids, PCI identity, the
station MAC drawn at random each boot (`src/station.rs`, locally administered
and unicast; with no randomness the radio stays down rather than fall back to
the efuse address), radio/link state, DMA ring metadata, the current channel,
and the session keys the supplicant installs for the active link.

Scanning is passive: the background sweep and the pre-join hunt listen for
beacons. The only frame sent before a join is a probe request for a network
the person marked hidden, naming that network alone
(`serve/connect/probe.rs`, proven in `connect_tests`). The scan list is aged
(a network unheard for three sweeps is dropped) and capped at sixteen.

## Runtime lifecycle

Startup discovers a supported Realtek Wi-Fi PCI function, claims it, maps its
BARs, allocates the TX/RX DMA rings, downloads firmware, reads the efuse
calibration, draws the per-boot station address and programs it into the MAC,
runs the MAC/BB/RF init tables, and serves IPC, polling the rings between
requests and sweeping channels 1 to 13 in the background.

A connect request hunts the target's beacon (probing only a hidden network),
then joins under the request's policy: SAE when the network offers it with
management frame protection capable, PSK otherwise, and never PSK for a
network the client saved as WPA3. The shared MLME runs authentication (SAE
commit and confirm, or open), association with the RSN element it chose, and
the four-way handshake, checking message 3's RSN element against the beacon;
the pairwise and group keys go into the hardware crypto engine. Received data
frames reach the stack only from the BSSID, protected, above the replay
counter; group rekeys are answered and installed; a deauthentication takes the
link down (unless PMF is on and it is unprotected). Teardown is handled by
process exit and broker revocation of the device, MMIO, and DMA grants.

## Failure model

Every setup phase rolls back prior broker grants on failure. Unsupported PCI
IDs, missing BARs, failed MMIO/DMA grants, firmware download
timeout, MAC/PHY init failure, association timeout, or a failed four-way prevent
the affected operation from reporting success. Packet transport above the link
layer stays in the upper network capsules.

With no RTL8821CE in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. The claim and register map are
retried on the shared bounded schedule (`nonos_libc::bring_up`: seven tries,
sleeping between them, each failed try giving the claim back); running out
leaves the capsule serving with stage `NotClaimed`, so the panel can show why
instead of the service vanishing. Later stages (power, firmware, MAC) are not
retried: the capsule serves the stage it reached, as before.

## Current implemented surface

- Realtek RTL8821CE PCI discovery and brokered device claim.
- Firmware download over the H2C/DDMA path with bounded DMA staging.
- A random per-boot station address programmed into the MAC id registers.
- MAC, baseband, and RF init tables with per-channel RF retune (2.4 GHz).
- TX and RX descriptor rings, polled; the FCS stripped and every descriptor
 length checked against its slot and the mapping.
- Hardware security engine with pairwise/group key install for CCMP, and the
 software CCMP path for frames the chip left encrypted.
- Passive scan with WPA2/WPA3 flags and signal; join with WPA3-SAE (H2E and
 hunting-and-pecking, PMF) or WPA2-PSK (PSK and PSK-SHA256) through the shared
 `nonos_wifi_core`; group rekey; deauthentication handling; protected leave.
- IPC status, scan, connect, link, and disconnect; net_core's link protocol.

## Wire format

Requests use the capsule header, version `1`, and the shared driver envelope.
Replies begin with a signed status word. All multi-byte integers are
little-endian.

## State ownership

`driver.rtl8821ce0` owns only hardware-facing Wi-Fi state: PCI identity, broker
grant ids, BAR mappings, DMA ring metadata, station MAC, radio and link state,
current channel, and the installed session keys. The Wi-Fi core owns scan
policy, association state machine, authentication, and key derivation;
`net.core` and above own addressing and transport.

## Operating rules

- Do not place DHCP, DNS, IP, or socket policy in this capsule.
- Do not import kernel driver or memory internals.
- Do not use inline PIO or architecture assembly.
- Do not persist Wi-Fi profiles, passphrases, or scan history.
- Firmware download must go through bounded DMA staging and explicit completion.

## Release target

The hardware chain is:

```text
driver.rtl8821ce0 -> nonos_wifi_core -> net.core -> net.ip -> apps
```

## Release evidence

- `rtl8821ce_proofs`: 152 `#[test]` functions, host only (116 passed in the
 earlier main run at,
 ).
 The crate includes `firmware/rtw8821c_fw.bin` with `include_bytes!`; without that file the
 crate does not compile.
- `nonos_wifi_core_proofs`: 70 `#[test]` functions, host only (14 in the earlier main run at
 ), including SAE and CCMP test
 vectors and full handshakes against a simulated access point.
- Lean `Nonos.Wpa2Handshake` builds (`docs/evidence/lean/nonos.log:224`). It is a model, not
 tied to this capsule's code.
- No hardware log and no QEMU log for this driver are committed. row 24
 marks scan, association, WPA2 and DHCP on hardware as not yet backed.

## Release checklist

- `rtl8821ce_proofs` and `nonos_wifi_core_proofs` stay green with the firmware binary present.
- A committed hardware serial log from an RTL8821CE machine shows device claim, firmware ready,
 MAC/BB/RF init, scan results, association and a completed WPA2 four-way.
- The same run shows a DHCP lease obtained through `net.core` and above.
- Capsule exit is shown to revoke the device, MMIO and DMA grants.
- Static gate passes for this README and the capsule manifest.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2), after one `no controller present, not started` line.
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- The power-on, efuse, firmware download and MAC stages each report on the console, and the panel shows the stage reached.
- If the claim or register map is refused, it is retried on the bounded schedule and the panel then shows `NotClaimed` rather than the service vanishing.
- Address: `[rtl8821ce] phy configured` follows the efuse read; `[rtl8821ce] no entropy
 for station address` instead means the kernel gave no randomness and the radio stays
 down. A capture shows the station's source address with the locally administered bit
 set, different on every boot.
- Scan: the Settings Wi-Fi panel status reads Ready and its scan counters climb (passes,
 raw frames, beacons; raw without beacons means frames arrive but do not parse). A
 WPA2-only AP shows WPA2, a WPA3-only AP shows WPA3, a transition-mode AP shows both.
 Only 2.4 GHz networks are listed.
- Nothing named on the air: a monitor-mode capture sees no probe request from the laptop
 while scanning or joining a visible network, and for a network saved as hidden one
 directed probe per channel naming only it.
- Join each AP type and read the connect reply (the panel shows its text; the reply
 carries the AKM: 2 PSK, 6 PSK-SHA256, 8 SAE):
 - WPA2-Personal (CCMP): joins with AKM 2; a wrong passphrase reads "The handshake did
 not finish; check the passphrase".
 - WPA3-Personal, hash-to-element only (SAE PWE 1): joins with AKM 8; a wrong password
 reads "WPA3: the access point did not accept the password".
 - WPA3-Personal, hunting-and-pecking (PWE 0): joins with AKM 8.
 - WPA3 transition mode: joins with AKM 8 (SAE preferred) and the network is saved as
 WPA3; if the AP later drops SAE the join is refused with "Saved as WPA3, but the
 network now offers only WPA2; not joined".
 - Open, TKIP-only or Enterprise: refused with "The network's security is not supported
 (open, TKIP or Enterprise)".
- Data: after a join, net_core binds the link and obtains a DHCP lease; the status
 counters show TX and RX-eth rising and receive refusals near zero. Leave the link up
 past the AP's group rekey interval: the rekey count rises and traffic continues.
- Leave: disconnect, or deauthenticate from the AP; the link reads down and the
 background scan resumes.

## Explicit non-goals today

No 5 GHz channels, no HT/VHT rates, no open or TKIP networks, no WPA2-Enterprise or 802.1X,
no access point or mesh mode, no monitor mode, no Bluetooth function of the combo chip, no
802.11ac rate tuning beyond the init tables, no power-save or roaming policy, no stored Wi-Fi
profiles, and no IP, DHCP, DNS or socket logic in this capsule.

## Verification

- Capsule: `(cd userland/capsule_driver_rtl8821ce && cargo build --release
 --target../x86_64-nonos-user.json -Zbuild-std=core,alloc
 -Zbuild-std-features=compiler-builtins-mem)`
- Proofs: `(cd userland/rtl8821ce_proofs && cargo test --release)` and
 `(cd userland/nonos_wifi_core_proofs && cargo test --release)`
- Build: `make -B nonos-mk-driver-rtl8821ce`
- Kernel profile: `cargo check --no-default-features --features
 microkernel-driver-rtl8821ce`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Handbook: [drivers](../../docs/handbook/drivers.md),
 [network stack](../../docs/handbook/network/stack.md).
