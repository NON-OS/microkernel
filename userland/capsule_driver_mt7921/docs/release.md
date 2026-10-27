# Lifecycle, failures, release and verification

Part of [capsule_driver_mt7921](../README.md).

## Runtime lifecycle

Spawned by the kernel only when the boot scan found one of its ids. It
claims the card, takes ownership, identifies the chip, reads and checks its
two package files, resets the subsystem, arms the rings, downloads the patch
and the RAM code, initialises the MAC, adds its interface and serves. It
serves whatever stage it reached, so the panels can say where it stopped.

## Failure model

Each failure is a stage (`src/stage.rs`): 2 power, 3 dead registers, 4
firmware, 5 no DMA, 7 no randomness, 8 another chip, 10 no package, 11 a
package that does not match. A firmware failure is retried three times with
a reset between; after the last the WFDMA is stopped and Bus Master turned
off. A card that stops answering ends the radio (`card stopped answering`).

## Release target

`microkernel-full-gui`, the Standard image, beside the RTL8821CE and iwlwifi
drivers; listed in `scripts/baselines/unsealed.txt` until the next seal.

## Release evidence

Host proofs only. No boot on a card.

## Release checklist

- [ ] A boot on an MT7922 (ThinkPad T14 Gen 4 AMD or an RZ616 card) and on
      an MT7921, by `docs/hardware/real-hardware-checklist.md`.
- [ ] A WPA3 join through this driver on a card (host-proven).
- [ ] The seal.

## Explicit non-goals today

Interrupts, power save, hardware scan, A-MPDU, CLC and SAR
power tables, 6 GHz, the MT7925, access point and monitor modes.

## Verification

`userland/mt7921_proofs`: the driver's own source over a modelled chip, its
MCU and a WPA2 access point (bring-up, download, channel grants, a join and
data). `verification/host_mutants/mt7921.json`: one mutant per rule, each
caught. `scripts/check_mediatek_firmware.py`: the package's pins agree
everywhere and no MediaTek byte is in the tree.
