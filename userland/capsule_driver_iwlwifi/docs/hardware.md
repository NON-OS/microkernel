## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the
parsers, the sequence against a model, and the bounded waits; these do not.
The `[iwlwifi]` lines below print only on a kernel that grants this capsule
`Debug`; without it, read the same facts from the status reply (stage, step,
detail, counters) and the Settings Wi-Fi panel.

- Absence and retry: on a machine without the device, no claim and exit
  `EXIT_ABSENT` (2), which the kernel names as `[EXIT] <service> status 2: no
  device present, not started` (`src/process/exit/end_note.rs`). With the
  device present but failing setup, at most seven claim and release rounds,
  then `EXIT_GAVE_UP` (6) and `[EXIT] <service> status 6: device present,
  bring-up failed and was given up`. The capsule holds no core while it waits.
- Setup on an SO laptop: the MAC clock comes ready in setup's legacy APM poll
  (it runs before the gen3 path takes the NIC; if it times out there, setup
  gives up and the gen3 path never runs).
- Identity: `[iwlwifi] pci 0x000051f0` (or your platform's id; the card's
  revision and RF id are no longer printed), then
  `[iwlwifi] firmware iwlwifi-so-a0-gf-a0-86.ucode` (or hr-b0-84).
- DMA: about ten grants in the broker log, none refused (else
  `[iwlwifi] no DMA region of N bytes` and stage NoDma).
- ALIVE: `[iwlwifi] firmware alive: status 0x0000cafe umac M.N sku 0x...`.
  If instead `radio not up: the firmware never raised ALIVE (step 7, detail
  0x00000030)`, the line before it gives the secure-boot CPU status, which
  says whether the ROM accepted the context info and the image loader.
- PNVM: with a non-zero SKU the boot waits up to 250 ms for PNVM_INIT_COMPLETE;
  detail 0x61 means it never came. With a PNVM file in the tree, the line
  `platform NVM section ... given` or `no platform NVM section for this SKU`
  says which happened.
- Up: `[iwlwifi] up: 38 channels, tx ant 3, regulatory 0x....`, then
  `[iwlwifi] radio up; passive scanning`. A failure names the group and
  command that went unanswered in its detail word.
- RF-kill: with the switch off, `the hardware RF-kill switch has the radio
  off`; scans are then not expected to complete.
- Scan: `[iwlwifi] first scan complete: N networks from M beacons` within
  about ten seconds; the Settings panel status reads Ready and its scan counters
  climb (sweeps, frames, beacons). Check against the APs in range: a WPA2-only
  AP shows WPA2, a WPA3-only (SAE, H2E or hunting-and-pecking) AP shows WPA3, a
  transition-mode AP shows both, and signals look plausible (-50 dBm or
  stronger reads 100).
- Nothing on the air: a monitor-mode capture next to the laptop sees no probe
  request from it during scans.
- Stalls and failures: `scan gave no completion within its budget` should not
  appear; `radio down: the firmware raised its error cause while running`
  means the firmware asserted.
- Joining available: neither `no randomness for a station address: scanning
  only` nor `joining off: the firmware runs another layout of command ...`
  appears. Before any join net_core logs `[NET-CORE] link probe
  driver.iwlwifi0 down` (no longer `no-answer`).
