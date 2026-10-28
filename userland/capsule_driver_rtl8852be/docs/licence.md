# Licences

The capsule's own source is AGPL-3.0-or-later, as all of NONOS. Two things
in it come from elsewhere, each with its notice beside it.

## The rtw89 tables (BSD-3-Clause)

`tables/*.bin` are register tables and calibration lists generated from
three files of Linux v6.12 `drivers/net/wireless/realtek/rtw89/`, each
`SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause`, Copyright (c)
2019-2022 Realtek Corporation:

- `rtw8852b_table.c`: the BB, BB gain, radio A and B, NCTL and TX power
  tables (`tables/gen.py`, `tables/gen_txpwr.py`);
- `rtw8852b_rfk_table.c` and `rtw8852b_rfk.c`: the RF calibration
  register lists (`tables/gen_rfk.py`).

NONOS takes them under the BSD-3-Clause option. The full notice is
`tables/LICENSE`, kept with the generated files; each generator checks the
sha256 of the source file it reads, and `scripts/baselines/prebuilt.txt`
marks the `.bin` files as generated. The Rust sources that follow rtw89's
functions (named in each file's header) are written for NONOS: they cite
rtw89 as the record of what the hardware needs, and copy no GPL-only code.

## The firmware (linux-firmware, rtlwifi licence)

`nonos-bootloader/firmware/realtek/rtw8852b_fw-1.bin` is Realtek's RTL8852B
firmware, 0.29.29.15, from linux-firmware's `rtw89/` directory. It is not
NONOS source and is not covered by the AGPL. It is redistributable under
linux-firmware's `LICENCE.rtlwifi_firmware.txt` (the `WHENCE` entry for
rtw89); the notice is `nonos-bootloader/firmware/realtek/LICENSE` and stays
with the binary. It is pinned by sha256
(`5b68415e3bfe72715d63a70703d4471b04b5475b8ff69cfdc8cdf48233cd5d3a`,
1245944 bytes) in `scripts/baselines/upstream.sha256`; see
`firmware/README.md`.
