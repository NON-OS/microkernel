#!/usr/bin/env python3
# NONOS Operating System (AGPL-3.0-or-later)
"""Write the RTL8852B parameter tables as data, from Linux v6.12's
drivers/net/wireless/realtek/rtw89/rtw8852b_table.c (GPL-2.0 OR BSD-3-Clause;
taken under BSD-3-Clause, notice in LICENSE beside this file).

    gen.py RTW8852B_TABLE_C OUT_DIR

Each rtw89_reg2_def array becomes a .bin of little-endian (addr u32,
data u32) pairs, in the source's order. The source is checked against the
sha256 below, so the data cannot drift from the file it names.
"""

import hashlib
import re
import struct
import sys
from pathlib import Path

SOURCE_SHA256 = "8b9b5fc32628037d8dec86cff976913164723d87abe38fd09d229c57b509b42f"
TABLES = {
    "rtw89_8852b_phy_bb_regs": "bb.bin",
    "rtw89_8852b_phy_bb_reg_gain": "bb_gain.bin",
    "rtw89_8852b_phy_radioa_regs": "radioa.bin",
    "rtw89_8852b_phy_radiob_regs": "radiob.bin",
    "rtw89_8852b_phy_nctl_regs": "nctl.bin",
}


def reg2(text, name):
    pat = r"rtw89_reg2_def\s+" + re.escape(name) + r"\[\]\s*=\s*\{(.*?)\n\};"
    body = re.search(pat, text, re.S).group(1)
    entry = r"\{\s*(0x[0-9A-Fa-f]+)\s*,\s*(0x[0-9A-Fa-f]+)\s*\}"
    pairs = re.findall(entry, body)
    # Nothing but entries, commas and whitespace may be left: no entry missed.
    if re.sub(r"[\s,]", "", re.sub(entry, "", body)):
        raise SystemExit(f"{name}: text the entry pattern does not cover")
    return [(int(a, 16), int(d, 16)) for a, d in pairs]


def main(src, out):
    raw = Path(src).read_bytes()
    if hashlib.sha256(raw).hexdigest() != SOURCE_SHA256:
        raise SystemExit("rtw8852b_table.c is not the v6.12 file this data comes from")
    text = raw.decode()
    for name, fname in TABLES.items():
        rows = reg2(text, name)
        data = b"".join(struct.pack("<II", a, d) for a, d in rows)
        Path(out, fname).write_bytes(data)
        print(f"{fname}: {len(rows)} entries, sha256 {hashlib.sha256(data).hexdigest()}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
