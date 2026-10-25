## Security a join accepts

The choice is made by `select` from the access point's RSN element and the flags (`userland/nonos_wifi_core/src/rsn/select.rs:87-134`).

- WPA3-Personal (SAE) whenever the access point offers it and can protect management frames (`userland/nonos_wifi_core/src/rsn/select.rs:100-105`, `Akm::Sae`).
- SAE runs over group 19 only, NIST P-256 (`userland/nonos_wifi_core/src/sae/group.rs:32`, `GROUP_19`), with the password element from hash-to-element or from hunting and pecking (`userland/nonos_wifi_core/src/sae/mod.rs:17-25`, `h2e`, `hnp`).
- WPA2-Personal otherwise: PSK-SHA256 when the access point offers it and can protect management frames, plain PSK if not (`userland/nonos_wifi_core/src/rsn/select.rs:117-132`, `PskSha256`).
- A network saved as WPA3 that now offers only WPA2 is refused as a downgrade (`userland/nonos_wifi_core/src/rsn/select.rs:106-116`, `Downgrade`).
- The pairwise and group cipher must be CCMP-128; TKIP is refused (`userland/nonos_wifi_core/src/rsn/select.rs:88-90`, `UnsupportedCipher`).
- Enterprise (802.1X), fast transition and OWE have no AKM here (`userland/nonos_wifi_core/src/rsn/select.rs:76-77`, `UnsupportedAkm`).
- Open networks and access points that admit only 802.11n stations are refused (`userland/nonos_wifi_core/src/mlme/failure.rs:26-37`, `OpenNetwork`, `NeedsHt`).

## Which access point a join takes

The hunt hears every beacon and probe response for the network before a join starts, and each becomes a candidate if this station could join it as it claims to be; the rule is the join's own (`userland/nonos_wifi_core/src/mlme/beacon/judge.rs`, `judge`). Candidates rank by security (SAE, then PSK-SHA256, then PSK), then signal, then how often each was heard (`userland/nonos_wifi_core/src/bss/candidate.rs`, `Candidate::key`). A frame naming a known BSSID with other RSN elements is a candidate of its own, so a forged beacon never replaces the access point's (`userland/nonos_wifi_core/src/bss/list.rs`, `Candidates::hear`). At most eight are kept and a better one replaces the worst (`MAX_CANDIDATES`). A join the access point ended (a refusal, silence, a password it could not prove, message 3 elements unlike its beacon) goes on to the next, at most three joins in all, each under fresh randomness (`userland/nonos_wifi_core/src/bss/next.rs`, `moves_on`; `userland/nonos_wifi_core/src/bss/list.rs`, `MAX_TRIES`). A failure on this side ends the request. A network heard only as one that cannot be joined answers with the reason the last such frame gave (`userland/nonos_wifi_core/src/bss/take.rs`, `Candidates::refusal`).

On a joined link the access point may run the four-way handshake again for a new pairwise key. A message 1 is answered while the keys in use carry on; a message 3 naming a new ANonce is checked under the PTK of that ANonce, and only if it verifies does that key take over, after message 4 has gone out under the old one, with packet numbers and replay counters started afresh (`userland/nonos_wifi_core/src/wpa/supplicant/rekey.rs`, `on_rekey_message3`; `userland/nonos_wifi_core/src/station/rekey.rs`, `LinkStation::set_pairwise_key`). Nothing waits between message 1 and message 3, so a forged message 1 costs one message 2 and can neither change a key nor stop the access point's rekey. The rekey's SNonce is derived under the current KCK and KEK from the ANonce, a seed of fresh randomness and the count of rekeys finished; the join's randomness gives the first seed and the driver draws a new one after each rekey that finishes (`userland/nonos_wifi_core/src/wpa/supplicant/seed.rs`, `Supplicant::reseed_rekey`).

The core's frame parsers, the join's judge and candidate list, the supplicant, SAE and a joined link are fed noise and mutated well-formed frames from a seeded generator (`userland/nonos_wifi_core_proofs/src/fuzz.rs`, `Rng::mutate`; `parser_fuzz_tests/`, `supplicant_fuzz_tests.rs`, `mlme_tests/fuzz_judge.rs`, `mlme_tests/fuzz_sae.rs`, `receive_tests/fuzz.rs`). Nothing may panic, a parser's answer must lie inside its frame, and no frame altered where a MIC or confirm covers it may be answered, change a key or carry a join forward. A beacon whose SSID element is longer than 32 octets is not a beacon (`userland/nonos_wifi_core/src/dot11/parse/beacon.rs`, `parse_beacon`).

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
