# nonos_wifi_core

`nonos_wifi_core` holds the chip-independent half of a Wi-Fi driver, so the
Intel (`capsule_driver_iwlwifi`) and Realtek (`capsule_driver_rtl8821ce`)
drivers share one copy of the 802.11, WPA2 and WPA3 logic and the link to
`net.core`. It touches no register and owns no ring. A driver implements two
traits over its own hardware, `LinkPort` and `KeyStore`, and calls into this
crate for everything else. It is `no_std` for the drivers and builds with std
under `cargo test`.

## What is in it

| Module | What it does |
|---|---|
| `dot11` | the 802.11 MAC header, management frames for scan, authentication and association, beacon elements, data frames with CCMP protection |
| `rsn` | reads what a beacon offers (RSNE, RSNXE), chooses SAE or PSK under the person's policy (`select`), builds the station's own elements |
| `sae` | WPA3-Personal SAE over group 19 (NIST P-256), station side: the password element by hash-to-element (`h2e`) when the access point offers it, else hunting and pecking (`hnp`); commit, confirm, KCK, PMK and PMKID |
| `wpa` | PSK and PTK derivation, the AKMs (WPA2-PSK, PSK-SHA256, SAE), HMAC, HKDF and SHA-256, and the supplicant that runs the four-way and group key handshakes |
| `eapol` | the EAPOL-Key codec, KDEs and MICs |
| `ccmp` | AES, AES-CCM, AES-CMAC and AES key unwrap, for chips without hardware CCMP |
| `mlme` | the association state machine: beacon, authentication (Open System or SAE), association, then the handshake, emitting the frames to send |
| `station` | the data path: ethernet to 802.11 and back, CCMP in software or hardware, the transmit sequence and packet numbers, a check of every received frame against the association (BSS, protection, replay), and the management frames of a live link (deauthentication under PMF, the SA Query) |
| `scan_list` | the networks a scan heard, deduplicated per access point, hidden ones dropped, aged by sweep, encoded for the Settings panel with WPA2 and WPA3 flags |
| `regdom` | the regulatory domain: wireless-regdb rules, the world domain as the default, Annex E channel frequencies, the Country element (narrows, never widens) and the scan plan that probes only where initiating radiation is allowed |
| `pmf` | management frame protection: the MMIE, BIP-CMAC-128, and the SA Query frames and procedure the station runs on a protected link |
| `netif` | the link protocol `net.core` speaks to a NIC, served over a `LinkPort`, so DHCP, DNS and TCP run over Wi-Fi as over a wired card |
| `key`, `frame` | the key-installation seam and the plaintext frame contract between the core and a driver |

A network saved as WPA3 is joined with SAE or not at all, and the station
listens only to the access point it chose. A join runs under the world
regulatory domain unless the driver is given another: an access point on a
channel the domain closes is treated as not heard.

## Dependencies

`p256` (with `expose-field` for the SAE maps), `sha2` and `hmac`, from
RustCrypto, the same versions and features the TLS client and `capsule_crypto`
build for the capsule target.

## What it does not do

- No enterprise (802.1X) authentication, no WPA3-SAE groups other than 19, no
  fast transition or roaming.
- No access point mode.
- No firmware, rings or DMA: those are each driver's.

## Tests

`userland/nonos_wifi_core_proofs` runs the crate on the host: RSN parsing and
selection, SAE against the IEEE Std 802.11-2020 Annex J.10 vectors, the
handshakes against a simulated access point, the MLME, the receive checks,
the regulatory domain, BIP and the SA Query, and the `netif` protocol. `rtl8821ce_proofs` and `iwlwifi_proofs` use it through
their drivers.

See [drivers](../../docs/handbook/drivers.md) and
[the network stack](../../docs/handbook/network/stack.md).
