# Wi-Fi, 1 of 2: iwlwifi and rtl8821ce

The two Wi-Fi drivers. The shared station core and the client library are in
wifi-2.md. Index and notation: README.md.

Of the nine Wi-Fi highs, the verifiers kept three, which are one bug: no link
loss is ever detected (W1, fixed once in nonos_wifi_core). Six are real but
documented gaps, now medium. Neither driver has a hardware or QEMU log.

## capsule_driver_iwlwifi: partial

Intel Wi-Fi: a radio only on AX210-family SO platforms (Alder and Raptor Lake CNVi, GF or HR modules), passive scan, WPA2-PSK and WPA3-SAE through nonos_wifi_core. Paths under userland/capsule_driver_iwlwifi/src/ unless they start with userland/.

- **high** `firmware/gen3/join/link.rs:94`: no beacon-loss detection, SA Query or IGTK, so a WPA3 link stays up and dead after an AP reboot or leaving range. A firmware-decrypted protected unicast deauth is handled, without a replay check. -> W1
- **medium**, was high (a documented non-goal that fails closed; no security impact) `server/radio/bring.rs:99`: AX200, AX201 on Qu/QuZ, 9260/9560, 8265, 7265, discrete AX210 and Meteor Lake are claimed with IRQ and DMA and never brought up, and the card-writing legacy ops stay open on them. -> W4
- **medium**, was high (availability only, on a driver never run on silicon; high once people rely on it) `server/radio/mod.rs:189`: a firmware assert or hardware error takes the radio down until reboot; the error tables are never read. -> W4
- **medium**, was high (works at short range; no security impact) `firmware/gen3/station/rates.rs:108`: legacy rates only, fixed per frame at the highest basic rate, with no rate control and no HT, VHT or HE. -> W5
- **medium** `firmware/gen3/join/target.rs:49`: the beacon's DS channel, beacon interval and DTIM go into firmware contexts unvalidated. -> W3
- **medium** `firmware/gen3/cmdq.rs:86`: one shared command DMA buffer is reused after a command times out, and the command ring is never checked for space. -> W4
- **medium** `firmware/gen3/join/bss.rs:159`: firmware-chosen TX queue numbers are trusted (0, out of range or duplicate). -> none (near W4)
- **medium** `server/radio/mod.rs:159`: firmware that stops answering without raising an error cause is never declared dead; retries are uncapped and block the serving loop. -> W4
- **medium** `server/radio/bring.rs:186`: one random station address per boot is reused for every network joined. -> W6
- **medium** `firmware/gen3/join/run.rs:120`: pairwise and group keys are copied and never zeroized, and the last GTK stays in device-visible DMA memory. -> W6
- **refuted**, was medium (only net.core and the Settings and setup panels may send to it, and the keys returned derive from the caller's own input) `server/handlers/connect.rs:80`: legacy NIWF ops return TK, GTK and PTK in IPC replies. -> W2 deletes them
- **medium** `firmware/blob.rs:19`: about 4,100 lines of dead legacy code and about 6.3 MB of never-booted firmware are linked and IPC-reachable. -> W2
- **medium** `userland/iwlwifi_proofs/src/harden_tests.rs:34`: hostile-input fuzzing covers only the legacy parsers; the gen3 parsers and the radio glue are untested. -> none
- **medium** `firmware/blob.rs:60`: no platform NVM (PNVM) ships for the AX211 GF radio. -> none
- **medium** `firmware/gen3/cmds.rs:112`: power save is off, interrupts are unused and receive is polled. -> none
- **medium** `firmware/gen3/join/hunt.rs:70`: it joins the first BSS heard for the SSID; no RSSI-based choice, roaming or PTK rekey. -> none
- low: `firmware/load.rs:86` safe functions dereference raw addresses without SAFETY comments; `server/radio/mod.rs:76` 63 files over 75 lines, code in mod.rs; `firmware/gen3/join/hunt.rs:79` loops bounded only by the clock or only by a count; `server/runner.rs:84` passphrase left in the receive buffer after a refused connect; `setup/sequence.rs:36` IRQ bound but never serviced; `firmware/gen3/dev.rs:148` command replies matched by sequence only; `server/radio/bring.rs:206` RF-kill read once at bring-up.

## capsule_driver_rtl8821ce: partial

Realtek RTL8821CE Wi-Fi function, 2.4 GHz only, polled, passive scan, WPA2 and WPA3-SAE through nonos_wifi_core. Paths under userland/capsule_driver_rtl8821ce/src/ unless they start with userland/.

- **high** `link.rs:331`: link loss is never detected; with PMF every deauth, protected ones included, is ignored, and there is no beacon-loss or SA Query, so the session and the stalled background scan stay forever. -> W1
- **medium**, was high (a certification blocker rather than a confidentiality or integrity flaw; channel 14 needs an attacker or a channel-14 AP) `serve/connect/hunt.rs:79`: no regulatory domain and a fixed uncalibrated TX power; hidden-network probes go out on channels 12 and 13, and a beacon's DS element can take it to 14. -> W3
- **medium** (new, found by the verifier) `serve/connect.rs:180`: after a join the radio is retuned to the beacon's raw DS byte (0, or a 5 GHz number), bypassing the hunt's clamp and leaving the link off-channel. -> W3
- **medium**, was high (works on 2.4 GHz legacy and mixed networks and refuses others visibly) `link.rs:409`: an 802.11ac card is driven at a fixed 6 Mb/s, 2.4 GHz legacy only, with no RSSI. -> W5
- **medium** `fwload.rs:147`: the firmware staging DMA is mapped without DMA32 or COHERENT, and an address above 4 GiB is truncated into the descriptor. -> none (near W7)
- **medium** `fw/dma.rs:72`: the safe DMA write wrapper does no bounds check, and grant lengths are never checked against the request. -> W7
- **medium** `main.rs:144`: MAC and HCI RX DMA are enabled before the RX ring base is programmed. -> none (near W7)
- **medium** `serve/mod.rs:101`: the passphrase and key copies are never wiped. -> W6
- **medium** `serve/radio.rs:160`: the station MAC is random per boot only, not per network or connection. -> W6
- **medium** `phy/mod.rs:29`: no H2C or C2H path and no RF calibration in the shipped build; the tested code does not ship. -> W5 (H2C); calibration: none
- **medium** `phy/apply.rs:43`: RF and BB table settle delays are a 4096-iteration spin instead of rtw88's 50 ms. -> W7
- **medium** `serve/connect/hunt.rs:71`: a connect busy-spins the single service loop for up to about 22 s. -> none
- **medium** `pcie/mod.rs:48`: no power management: ASPM L1 and CLKREQ stay off, no LPS, polling every 100 ms. -> none
- **medium** `rx/poll.rs:74`: a dead or removed card is not detected; all-ones reads replay stale RX slots forever. -> none (near W7)
- **medium** `userland/rtl8821ce_proofs/src/lib.rs:46`: the serve, control and hunt paths are not mounted, and there is no randomized hostile input. -> none
- **medium** `userland/capsule_driver_rtl8821ce/Cargo.toml:20`: over-the-air parsing and the supplicant run in the process that holds MMIO and DMA. -> none
- low: `regs.rs:47` MMIO accessors without SAFETY comments or bounds; `link.rs:365` broadcast probes and management frames flagged unicast; `serve/mod.rs:70` 34 files over 75 lines, code in mod.rs; `serve/control/status_op.rs:59` status discloses the MMIO address and efuse registers; `serve/mod.rs:143` the retired GTK stays in the CAM; `assoc.rs:194` EAPOL found by pattern search; `pwr/poll.rs:26` waits bounded by read counts, DBI failures reported as success; `fwload.rs:83` DMA grants never unmapped.
