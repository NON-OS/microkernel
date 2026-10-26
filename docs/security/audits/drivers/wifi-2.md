# Wi-Fi, 2 of 2: nonos_wifi_core and nonos_wifi_client

The station core both drivers share, and the client library the panels and
net.core use. The drivers are in wifi-1.md. Index and notation: README.md.

## nonos_wifi_core: partial

Chip-independent 802.11, WPA2-Personal and WPA3-SAE station logic (MLME, RSN selection, SAE group 19, handshakes, software CCMP, data receive checks). The verifiers found no memory-safety or authentication-bypass path from the air. Paths under userland/nonos_wifi_core/src/ unless they start with userland/.

- **high** `station/receive.rs:103`: management frame protection is negotiated but its receive side is absent (no SA Query, no BIP, no protected deauth path, the IGTK unused); this is the root cause behind iwlwifi link.rs:94 and rtl8821ce link.rs:331. -> W1
- **medium**, was high (a documented non-goal that fails closed with a clear code) `mlme/beacon.rs:50`: Enterprise (802.1X/EAP), open and OWE networks cannot be joined. -> none (plan/55 W5: "follow as their own unit")
- **medium** `wpa/supplicant/state.rs:71`: all key material (PMK, PTK, GTK, IGTK, passphrase, SAE secrets, AES round keys) lives in Copy types with no Drop and is never zeroized. -> W6
- **medium** (confirmed; bounded by the WPA3-only pin) `sae/station.rs:153`: unauthenticated SAE frames end the exchange, and rtl8821ce then rejoins transition networks with WPA2-PSK on any SAE failure, a timeout included, which hands an eavesdropper message 2 for an offline guess. -> W2
- **medium** `ccmp/aes.rs:89`: software AES indexes S-box tables with secret state and branches on secret bits, a cache and timing side channel on TK, KEK and KCK. -> none
- **medium** `mlme/beacon.rs:40`: one unauthenticated beacon decides, and can end, the join; there is no choice among candidate BSSs. -> none
- **medium** `sae/station.rs:144`: the core state machines have no deadlines or retry caps, and anti-clogging round trips are unbounded. -> none
- **medium** `wpa/supplicant/step.rs:89`: PTK rekey is not supported; message 1 after Connected is dropped, so the AP deauthenticates at each rekey. -> none
- **medium** `eapol/kde.rs:86`: the Transition Disable KDE is ignored, so a transition network is never locked to SAE by the AP's own signal. -> none (near W2)
- **medium** `mlme/assoc.rs:63`: association is legacy only (no HT, VHT, HE or WMM elements), and HT-required APs are refused. -> W5
- **medium** `mlme/auth.rs:61`: no roaming, FT or PMKSA caching; TKIP-group mixed mode and SAE-EXT-KEY are refused. -> none
- **medium** `userland/iwlwifi_proofs/src/harden_tests.rs:13`: the core's own parsers and state machines have no fuzz or Kani coverage; the existing fuzz hits iwlwifi's fork. -> none
- **low**, was medium (only net.core and the panels reach it, with their own secrets, and the fork cannot transmit) `userland/capsule_driver_iwlwifi/src/main.rs:28`: iwlwifi compiles a forked, older supplicant without the downgrade check and exposes it over IPC. -> W2
- low: `wpa/supplicant/state.rs:133` Supplicant::new skips the message 3 RSNE check (confirmed, no live caller, W2); `rsn/select.rs:131` PMF not negotiated on capable WPA2 networks; `eapol/kde.rs:105` GTK key index 0 accepted; `wpa/supplicant/keys.rs:54` GTK compared in non-constant time; `station/mod.rs:194` CCMP packet number wraps instead of refusing at 2^48; `netif/serve.rs:53` OP_TX_PACKET payload not bounded; `mlme/state.rs:134` an overlong SSID silently truncated; `dot11/ies.rs:67` the element walk hides valid elements before a malformed one; `station/mod.rs:49` 36 files over 75 lines, code in mod.rs.

## nonos_wifi_client: near

Library for Settings, the setup wizard and net.core: finds the Wi-Fi driver, speaks its control protocol, and keeps up to four saved networks sealed under a TPM-derived key. Paths under userland/nonos_wifi_client/src/.

- **medium** `saved/file.rs:23`: the sealed saved-networks record has no generation counter, so an older one can be restored, bringing back a forgotten network or clearing a WPA3-only flag. -> none
- **medium** `saved/store.rs:64`: the WPA3-only downgrade guard fails open on any load error, so a network saved as WPA3 is joined without it. -> none (near W2's SAE pin)
- **medium** `join_wire.rs:69`: a passphrase over 64 bytes is silently cut to 64 and sent as a different credential. -> none
- low: `driver/call.rs:56` replies not checked against the request's magic, op and id (W6); `driver/stage.rs:68` firmware_step can never read a step from rtl8821ce; `saved/write.rs:28` the saved list is replaced non-atomically; `wire.rs:69` panel proofs exercise a stale model, files over 75 lines.
