## Security a join accepts

The choice is made by `select` from the access point's RSN element and the flags (`userland/nonos_wifi_core/src/rsn/select.rs:87-134`).

- WPA3-Personal (SAE) whenever the access point offers it and can protect management frames (`userland/nonos_wifi_core/src/rsn/select.rs:100-105`, `Akm::Sae`).
- SAE runs over group 19 only, NIST P-256 (`userland/nonos_wifi_core/src/sae/group.rs:32`, `GROUP_19`), with the password element from hash-to-element or from hunting and pecking (`userland/nonos_wifi_core/src/sae/mod.rs:17-25`, `h2e`, `hnp`).
- WPA2-Personal otherwise: PSK-SHA256 when the access point offers it and can protect management frames, plain PSK if not (`userland/nonos_wifi_core/src/rsn/select.rs:117-132`, `PskSha256`).
- A network saved as WPA3 that now offers only WPA2 is refused as a downgrade (`userland/nonos_wifi_core/src/rsn/select.rs:106-116`, `Downgrade`).
- The pairwise and group cipher must be CCMP-128; TKIP is refused (`userland/nonos_wifi_core/src/rsn/select.rs:88-90`, `UnsupportedCipher`).
- Enterprise (802.1X), fast transition and OWE have no AKM here (`userland/nonos_wifi_core/src/rsn/select.rs:76-77`, `UnsupportedAkm`).
- Open networks and access points that admit only 802.11n stations are refused (`userland/nonos_wifi_core/src/mlme/failure.rs:26-37`, `OpenNetwork`, `NeedsHt`).

On a joined link a received frame reaches the stack only from the access point, protected and above the replay counter; fragments and A-MSDUs are dropped (`userland/nonos_wifi_core/src/station/receive.rs:47-61`, `RxDrop`).

## What a join answers

The panel turns the driver's code into one line with `join_text` (`userland/nonos_wifi_client/src/driver/join_text.rs:15-36`).

| Code | Panel text |
|---|---|
| 0 | `Joined` |
| -1 | `The radio is down or the request was malformed` |
| -2 | `The network was not heard on any channel` |
| -3, -4 | `The keys could not be installed in the card` |
| -5 | `The access point refused the association` |
| -6 | `The handshake did not finish; check the passphrase` |
| -7 | `Saved as WPA3, but the network now offers only WPA2; not joined` |
| -8 | `The network's security is not supported (open, TKIP or Enterprise)` |
| -9 | `A passphrase is 8 to 63 characters, or 64 hex digits` |
| -10 | `WPA3: the access point did not accept the password` |
| -11 | `The handshake did not match the network's beacon; not joined` |
| -12 | `No randomness for the handshake; not joined` |
| -38 | `This driver cannot join networks yet`, the code `CANNOT_JOIN` |
| -101 | `The driver did not answer`, the code `NO_REPLY` |
| any other | `The join failed` |

The client answers -38 itself, and sends no passphrase, for a driver its table marks as unable to join (`userland/nonos_wifi_client/src/driver/connect.rs:60-64`, `joins`). Both drivers are marked as able to join in 0.9.2 (`userland/nonos_wifi_client/src/driver/services.rs:32-35`, `SERVICES`), so every join reaches the driver, and the iwlwifi driver answers -38 itself when its radio cannot join.
