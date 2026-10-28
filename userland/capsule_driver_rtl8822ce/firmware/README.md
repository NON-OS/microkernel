# RTL8822CE firmware

`rtw8822c_fw.bin` is Realtek RTL8822C device firmware (version 9.9, H2C
format 15), downloaded to the chip at init. It is NOT part of the NONOS
source and is NOT covered by the AGPL: it is a redistributable binary from
the linux-firmware project. The capsule links the copy the tree already
carries, `nonos-bootloader/firmware/realtek/rtw8822c_fw.bin`, with
`include_bytes!` (`src/fwload/image.rs`).

- Source: linux-firmware, `rtw88/rtw8822c_fw.bin`
  https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git
- Size and pin: 202600 bytes, sha256
  `3deecb31210986d98cdbfb000391e08d602a6eee4ffc883969faa2b907ab03ba`
  (`scripts/baselines/upstream.sha256`; `rtl8822ce_proofs`
  `fw_tests::the_image_is_the_pinned_one` fails on any other bytes).
- Copyright: Realtek Semiconductor Corp.
- License: redistributable per the linux-firmware `WHENCE` entry for
  `rtw88` (Realtek's redistribution terms,
  `nonos-bootloader/firmware/realtek/LICENSE`). Retain this notice with the
  binary.
