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
| `eapol` | the EAPOL-Key codec, KDEs and MICs. The Wi-Fi Alliance's Transition Disable KDE is read too, and the supplicant keeps its bits only from a message 3 that verified, unwrapped under the PTK and passed the downgrade check; the join report and `Mlme::transition_disable` hand them to the driver, which keeps them with the saved network |
| `ccmp` | AES-CCM and AES key unwrap, for chips without hardware CCMP. AES and AES-CMAC are the system's, from `nonos_aes`, and are re-exported here as `ccmp::aes` and `ccmp::cmac` |
| `mlme` | the association state machine: beacon, authentication (Open System or SAE), association, then the handshake, emitting the frames to send |
| `station` | the data path: ethernet to 802.11 and back, CCMP in software or hardware, the transmit sequence and packet numbers, a check of every received frame against the association (BSS, protection, replay), and the management frames of a live link (deauthentication under PMF, the SA Query). A driver that feeds every frame to `receive` and ticks `tick` gets the SA Query asked and run for it, an access point gone quiet probed and then lost, and a pairwise rekey taken: the new PTK only after an authentic message 3, message 4 under the old key |
| `scan_list` | the networks a scan heard, deduplicated per access point, hidden ones dropped, aged by sweep, encoded for the Settings panel with WPA2 and WPA3 flags |
| `regdom` | the regulatory domain: wireless-regdb rules, the world domain as the default, Annex E channel frequencies, the Country element (narrows, never widens) and the scan plan that probes only where initiating radiation is allowed |
| `pmf` | management frame protection: the MMIE, BIP-CMAC-128, and the SA Query frames and procedure the station runs on a protected link. Beacon protection too: with a BIGTK from the handshake, the session AP's beacons count only if their MME verifies (BIP with the Timestamp taken as zero) under the key's index and their BIPN climbs; one that fails is counted in the status reply and otherwise ignored, liveness included. A station that verifies beacons (`Mlme::verify_beacons`, which the soft-MAC station calls) sets Extended Capabilities bit 84 in its (re)association request when management frame protection is negotiated |
| `softmac` | the whole station above a soft-MAC chip: a driver implements `softmac::Radio` (tune, send and receive whole frames, point the filter at a BSS) and `Env` (clock, randomness), and `SoftStation` runs the background scan, the join (hunt, the state machine with retransmission, the WPA3-to-WPA2 transition retry), the session (CCMP in software, PMF, SA Query, loss of the AP), the Wi-Fi control family and the net_core link protocol, on the same wires the existing drivers answer |
| `pmksa` | the PMKSA cache (IEEE Std 802.11-2020, 12.6.10.3): the PMK and PMKID an SAE join made with one AP, kept for this boot only and never written anywhere, at most four, each for one AP, network and passphrase, offered for the PMK lifetime's default of 12 hours and no longer. Entries are keyed by station address too, and name their passphrase by an HMAC keyed with 32 octets drawn from the station's randomness when it starts: an SSID-keyed tag would be an offline dictionary oracle for the password. The tag key and every tag live in a `vault`, never logged and never leaving the driver; a station without randomness keeps no PMKSA. Every entry removed, replaced or outlived is wiped, and the person's disconnect wipes them all. A join to an AP it holds a PMKSA for runs Open System authentication and lists the PMKID in the RSNE instead of running SAE. If the AP refuses, the entry is forgotten and SAE runs |
| `softmac::roam` | roaming within the session's ESS (IEEE Std 802.11-2020, 11.3.5.4). The session averages its AP's signal, from frames that verify under the session's keys only, since anyone can send a beacon in the AP's name: protected data, and beacons verified under beacon protection. Once that has stayed below -75 dBm (and not within 30 s of the last look or move), and while the link runs no SA Query or rekey, a passive look runs, nothing of the session going out while the station is away: never a probe, every other channel of the domain's plan, two at a time with 300 ms home between, the AP told the station is away (a Null frame with Power Management, 11.2.3) and back. An AP of the same SSID, from whose RSNE the session's own policy selects exactly the `Selection` it runs, naming the same group management cipher, not barred, and heard at least 8 dB stronger, is moved to by a Reassociation Request naming the AP left, on its cached PMKSA if one is held, with fresh randomness and message 3 held to the candidate's own beacon. The session keeps only the secret its AKM needs, in a `vault` wiped when it ends: the PMK for PSK, the password token for SAE by hash-to-element, the password for hunting and pecking. A move the new AP refuses leaves the station on its old AP; one accepted whose handshake fails ends the session. Either bars the candidate for the boot and reports its own code in the status reply, never "wrong password" |
| `address` | the station's addresses: one per network, an HMAC under a secret drawn once per boot over the SSID and the request's security mode, so a reconnect keeps it and two networks cannot link the machine; a fresh one for each scan that transmits (a hidden network's directed probes), never the last scan's or the network's; all locally administered and individual. A radio that cannot take one ends the join (`CODE_NO_ADDRESS`), and a move to another AP is not made from a radio that no longer holds the session's address, as after a chip reset: the session ends instead. Nothing of the secret outlives the boot |
| `bss` | the access points a hunt heard for one network, judged by the MLME's own rule (`mlme::judge`) and ranked by security, then signal: a join takes the best and, if the access point ends it (refused, silent, left, or a password it could not prove), the next, at most three times, so no single beacon decides or ends a join |
| `link_history` | why a driver's link last ended (lost or left) and how its last join came out, for the link reply; a rejoin is refused once the person left |
| `vault` | secrets kept past one exchange: boxed so they never move, neither `Copy` nor `Clone` nor `Debug` (checked when the crate builds), wiped with volatile writes when dropped. The MLME, the supplicant, the SAE exchange, the randomness and the link's keys wipe themselves the same way |
| `netif` | the link protocol `net.core` speaks to a NIC, served over a `LinkPort`, so DHCP, DNS and TCP run over Wi-Fi as over a wired card; `gate` admits only net.core's owner to it |
| `key`, `frame` | the key-installation seam and the plaintext frame contract between the core and a driver |

A network saved as WPA3 is joined with SAE or not at all, and the station
listens only to the access point it chose. A join runs under the world
regulatory domain unless the driver is given another: an access point on a
channel the domain closes is treated as not heard.

## Dependencies

- `nonos_aes`: AES-128, AES-256 and AES-128-CMAC, the system's one of each,
  with a computed S-box and AES-NI where the CPU has it.
- `nonos_p256`: NIST P-256, the system's one, which SAE's group 19 is. The
  Bluetooth Security Manager uses the same two crates.
- `nonos_mac`: the bits that make an address local and unicast, every
  network driver's.
- `sha2` and `hmac`, from RustCrypto, the same versions and features the TLS
  client and `capsule_crypto` build for the capsule target.

## What it does not do

- No enterprise (802.1X) authentication, no WPA3-SAE groups other than 19, no
  fast transition (roaming within an ESS is by reassociation).
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

## Keys and the chip

A soft-MAC chip is given no key: `Radio` has no way to take one, and the
station seals every data frame itself. A restarted chip therefore cannot
send a link's data in the clear.

A driver that keys a `LinkStation` with `Ccmp::Hardware` hands the chip
plaintext for it to encrypt. The station refuses every such frame until
the driver says, with `chip_keys(true)`, that the chip holds the keys. It
refuses them again from `chip_keys(false)` until they are back:
- a firmware restart;
- a key install that failed or was never answered;
- a key cache cleared.

Only the EAPOL a handshake sends before any key is in place goes in the
clear.

Every orderly end wipes the station's keys: the pairwise key, the group
keys, the IGTK, the BIGTK, and the supplicant that holds them.
`holds_keys` says whether any remain. A PMKSA comes only of a handshake
that finished.

## Known attack classes

- **Key reinstallation.**
  - A message 3 sent again is answered with message 4 and installs
    nothing.
  - A group key already in its slot keeps its replay counter, even when
    the AP hands it back after rotating away from it.
  - A repeated group key is never handed to the driver to install again.
  - No GTK, IGTK or BIGTK of all zeros is installed.
- **Fragments.** None is joined. Every frame with More Fragments set or
  a fragment number other than zero is refused, sealed or in the clear,
  so there is no fragment cache.
- **A-MSDU.** None is taken. The station negotiates no SPP A-MSDU, so the
  A-MSDU Present bit is not under the MIC, and every QoS frame with it
  set is refused.
- **EAPOL.** EAPOL goes to the supplicant, never to the stack. EAPOL to a
  group address is refused.
- **Channel switch.** No Channel Switch Announcement is acted on, in a
  beacon or an action frame, protected or not. When the AP really
  switches, the link ends on missed beacons and the station joins again
  by scanning.

## SAE against a hostile AP

- **The AP's commit is checked before anything is computed from it:**
  - its group must be 19;
  - its scalar must lie in 1 < s < r;
  - its element must be on the curve, which the identity never is.
- **A reflected commit** (this station's own) is dropped.
- **Only group 19 is run.** An AP that asks for another group, weaker or
  not, ends the exchange.
- **Hash-to-element is used whenever the AP's RSNXE offers it.**
- **Hunting and pecking**, for an AP that does not offer hash-to-element,
  runs exactly 40 rounds, whatever the password, and selects in constant
  time. A password with no element in them (a chance near 2^-40 for one
  pair of addresses) is refused rather than hunted for longer.
- **Anti-clogging requests** are answered at most three times.
- **The AP's commit sent again** after this station confirmed draws its
  confirm again, the send-confirm counter one higher, at most three
  confirms in all. Any other commit then is ignored.
- **No PMK** is out before the AP's confirm verifies.
