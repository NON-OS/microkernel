# nonos_wifi_core_proofs

Host tests for `nonos_wifi_core`, the chip-independent half of the Intel and
Realtek Wi-Fi drivers. It depends on `nonos_wifi_core` by path and calls its
public API. No hardware and no chip driver are involved. Everything in `src/`
is `#[cfg(test)]`, so the library is empty outside a test build.

## What is tested

70 tests:

| File | Tests | What it checks |
|---|---|---|
| `crypto_tests.rs` | 3 | AES-128-CMAC against RFC 4493, HMAC-SHA256 and HKDF against RFC 5869 A.1, and the PSK from 64 hex digits or a passphrase through PBKDF2 (IEEE 802.11-2020 J.4.2) |
| `sae_tests.rs` | 9 | WPA3-SAE against IEEE 802.11-2020 Annex J.10 as transcribed in hostapd's `common_module_tests.c`, for hunting and pecking and for hash-to-element, then the exchange end to end |
| `handshake_tests.rs` | 9 | the supplicant against IEEE 802.11-2020 12.7.6 and 12.7.7: repeated message 1, RSNE downgrade in message 3, forged and replayed frames, repeated message 3 without a key reinstall, group key handshake, SAE AKM, larger key data |
| `supplicant_tests.rs` | 2 | the GTK keeps the key index the access point sent |
| `mlme_tests.rs` | 7 | WPA2 and WPA3-SAE joins against a simulated access point, a WPA3 network shown as PSK-only refused, foreign frames ignored, the reasons a join ends, and the rates offered in the association request |
| `rsn_tests.rs` | 8 | RSNE parsing and defaults, AKM choice, TKIP group cipher refused, the station's RSNE, and the key data walk |
| `receive_tests.rs` | 8 | CCMP receive against IEEE 802.11-2012 M.6.4 and a QoS vector, then which frames `LinkStation::receive` delivers or drops |
| `protect_tests.rs` | 5 | transmit-side CCMP and protected deauthentication, against IEEE 802.11-2012 M.9.2 |
| `station_tests.rs` | 5 | the station data path with hardware and software CCMP, counters, and no transmit while unassociated |
| `netif_tests.rs` | 7 | the `net.core` link protocol: MAC address, link status, transmit, receive and malformed requests, over a mock link |
| `scan_list_tests.rs` | 7 | the shared scan list: signal encoding, refresh, dBm to percent clamping, and malformed input dropped |

`ap_sim.rs` is the authenticator side of the four-way and group key
handshakes that the MLME and handshake tests drive the station against.

Two expected outputs have no published answer to check against: the protected
deauthentication MPDU in `protect_tests.rs` and the QoS CCMP frame in
`receive_tests.rs`. Both were computed with the Python `cryptography` AES-CCM,
as the file headers say.

## What it does not cover

It does not run either driver, touch hardware or test a chip's own CCMP
offload. AES-CCM itself against RFC 3610 is in `iwlwifi_proofs`. The
`cfg(kani)` lint is declared in `Cargo.toml`, but the crate has no Kani proofs.

## Running

`cargo test --release` from this directory. The crate ends in `_proofs` and
has a `Cargo.lock`, so `nix flake check` runs it as
`proofs-nonos_wifi_core_proofs`. Nothing depends on it. See
[Drivers](../../docs/handbook/drivers.md) and
[the proofs page](../../docs/handbook/verification/proofs.md).
