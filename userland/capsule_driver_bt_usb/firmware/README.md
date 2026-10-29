# RTL8821C Bluetooth patch

`rtl8821c_fw.bin` and `rtl8821c_config.bin` are Realtek's Bluetooth patch
and its configuration for the RTL8821C (the Bluetooth half of the
RTL8821CE combo card, on USB). The capsule sends them to a controller whose
ROM answers as an RTL8821C, before standard bring-up (`nonos_bt_rtl`). They
are NOT part of the NONOS source and NOT covered by the AGPL: they are
redistributable binaries from the linux-firmware project, kept here
unmodified.

- Source: linux-firmware, `rtl_bt/rtl8821c_fw.bin` and
  `rtl_bt/rtl8821c_config.bin`, at commit
  d0039fd797d7776d89f00c22c45754914a96706e
  (https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git).
- WHENCE, the rtl_bt block that lists both files: "Licence:
  Redistributable. See LICENCE.rtlwifi_firmware.txt for details."
- Licence: `LICENCE.rtlwifi_firmware.txt`, beside them, from the same
  commit. Copyright (c) 2010, Realtek Semiconductor Corporation.
- Pins: `SHA256SUMS` here, `scripts/baselines/upstream.sha256` for
  `scripts/check_prebuilt.py`, and `src/rtl/pins.rs`, which the proofs hold
  the embedded bytes to.

## Embedded, not packaged

The licence permits redistribution in binary form, without modification,
provided the copyright notice and disclaimer are reproduced with it; it
forbids reverse engineering and the use of Realtek's name to promote. It
puts no condition on what the files are distributed with. So they are
embedded in the capsule, as the RTL8821CE Wi-Fi firmware is
(`nonos-bootloader/firmware/realtek`), and this notice and the licence go
with the source.

MediaTek's firmware is different: `LICENCE.mediatek` allows redistribution
"but not as part of the Linux kernel or in any other form which would
require these files themselves to be covered by the terms of the GNU General
Public License". A copyleft capsule binary is such a form, so the MT7921
files are a package on the boot medium (`scripts/firmware_package.py`), never
in a capsule. Nothing in Realtek's licence asks for that.

The bytes are not altered in the tree. The patch's last four octets are
replaced by the file's own version only in the copy sent to the controller,
as the controller expects it (Linux v6.12 `drivers/bluetooth/btrtl.c`,
`rtlbt_parse_firmware`).

The loader is original NONOS code. The format and the commands it uses are
hardware facts read from Linux v6.12 `drivers/bluetooth/btrtl.c` and
`btrtl.h` (GPL-2.0-only); no code was copied.
