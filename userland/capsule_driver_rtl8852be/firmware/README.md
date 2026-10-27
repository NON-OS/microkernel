# RTL8852BE firmware

`nonos-bootloader/firmware/realtek/rtw8852b_fw-1.bin` is Realtek RTL8852B
device firmware (container format 1; normal and WoWLAN suits for cuts B and
C, firmware 0.29.29.15, plus its log-format entry), downloaded to the chip's
WiFi CPU at init. It is NOT part of the NONOS source and is NOT covered by the
AGPL: it is a redistributable binary from the linux-firmware project.

- Source: linux-firmware, `rtw89/rtw8852b_fw-1.bin`
  https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git
- Pinned: sha256 `5b68415e3bfe72715d63a70703d4471b04b5475b8ff69cfdc8cdf48233cd5d3a`,
  1245944 bytes, in `scripts/baselines/upstream.sha256`.
- Copyright: Realtek Semiconductor Corp.
- License: redistributable per the linux-firmware `WHENCE` entry for `rtw89`,
  under `LICENCE.rtlwifi_firmware.txt`; the notice is
  `nonos-bootloader/firmware/realtek/LICENSE`. Retain it with the binary.

Linux v6.12 asks for `rtw8852b_fw-1.bin` first (`RTW8852B_FW_FORMAT_MAX` 1)
and this driver takes the same file. Its suits use header format 0; the
driver picks the normal suit for the chip's cut (`src/fw/mfw.rs`) and refuses
any other. The file carries no firmware elements (no TX power or RF tables),
so the driver's tables are the ones Linux falls back to, in `../tables/`.
