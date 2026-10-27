#!/usr/bin/env python3
# NONOS Operating System (AGPL-3.0-or-later)
"""Write the RTL8852B calibration register lists, from Linux v6.12's rtw89
rtw8852b_rfk_table.c and rtw8852b_rfk.c (both GPL-2.0 OR BSD-3-Clause;
taken under BSD-3-Clause, notice in LICENSE beside this file).

    gen_rfk.py RTW8852B_RFK_TABLE_C RTW8852B_RFK_C OUT_DIR

rfk.bin starts with one u32 count per list, in the order of LISTS; then
every list's entries follow in that order, each five little-endian u32:
flag (rtw89_rfk_flag: 0 RF write, 1 BB mask write, 2 BB set, 3 BB clear,
4 delay in us), RF path, address, mask, data. The reg3 lists of
rtw8852b_rfk.c are BB mask writes. Both sources are checked against the
sha256 values below.
"""

import hashlib
import re
import struct
import sys
from pathlib import Path

TABLE_SHA256 = "ebc6840d2e15c7a9f12fb7d7121ca19bb8acff93524d3902a7ce892e0714fc21"
RFK_SHA256 = "c2916c132407b47e83336956938a53d654f497753cf67fe7df34057968e8afcd"
# rtw89 runs _check_dadc on path B only, so path A's two lists are not taken.
LISTS = [
    ("rtw8852b_afe_init_defs", "table"),
    ("rtw8852b_check_addc_defs_a", "table"),
    ("rtw8852b_check_addc_defs_b", "table"),
    ("rtw8852b_check_dadc_en_defs_b", "table"),
    ("rtw8852b_check_dadc_dis_defs_b", "table"),
    ("rtw8852b_dack_s0_1_defs", "table"),
    ("rtw8852b_dack_s0_2_defs", "table"),
    ("rtw8852b_dack_s0_3_defs", "table"),
    ("rtw8852b_dack_s1_1_defs", "table"),
    ("rtw8852b_dack_s1_2_defs", "table"),
    ("rtw8852b_dack_s1_3_defs", "table"),
    ("rtw8852b_set_nondbcc_path01", "rfk"),
    ("rtw8852b_restore_nondbcc_path01", "rfk"),
]


def num(s):
    s = s.strip()
    m = re.fullmatch(r"BIT\((\d+)\)", s)
    if m:
        return 1 << int(m.group(1))
    m = re.fullmatch(r"GENMASK\((\d+),\s*(\d+)\)", s)
    if m:
        hi, lo = int(m.group(1)), int(m.group(2))
        return ((1 << (hi + 1)) - 1) & ~((1 << lo) - 1)
    if re.fullmatch(r"RF_PATH_([AB])", s):
        return "AB".index(s[-1])
    return int(s, 0)


def body(text, name):
    m = re.search(re.escape(name) + r"\[\]\s*=\s*\{(.*?)\n\};", text, re.S)
    if not m:
        raise SystemExit(f"{name}: not found")
    return m.group(1)


def reg5(text, name):
    b = body(text, name)
    rows = []
    for kind, args in re.findall(r"RTW89_DECL_RFK_(WRF|WM|WS|WC|DELAY)\(((?:[^()]|\([^()]*\))*)\)", b):
        a = [num(x) for x in re.split(r",(?![^(]*\))", args)]
        if kind == "WRF":
            rows.append((0, a[0], a[1], a[2], a[3]))
        elif kind == "WM":
            rows.append((1, 0, a[0], a[1], a[2]))
        elif kind in ("WS", "WC"):
            rows.append(({"WS": 2, "WC": 3}[kind], 0, a[0], a[1], 0))
        else:
            rows.append((4, 0, 0, 0, a[0]))
    left = re.sub(r"RTW89_DECL_RFK_\w+\((?:[^()]|\([^()]*\))*\)", "", b)
    if re.sub(r"[\s,]", "", left):
        raise SystemExit(f"{name}: text the entry pattern does not cover")
    return rows


def reg3(text, name):
    b = body(text, name)
    entry = r"\{\s*(0x[0-9a-fA-F]+)\s*,\s*(0x[0-9a-fA-F]+)\s*,\s*(0x[0-9a-fA-F]+)\s*\}"
    if re.sub(r"[\s,]", "", re.sub(entry, "", b)):
        raise SystemExit(f"{name}: text the entry pattern does not cover")
    return [(1, 0, int(a, 16), int(m, 16), int(d, 16)) for a, m, d in re.findall(entry, b)]


def main(table_c, rfk_c, out):
    texts = {}
    for key, path, want in (("table", table_c, TABLE_SHA256), ("rfk", rfk_c, RFK_SHA256)):
        raw = Path(path).read_bytes()
        got = hashlib.sha256(raw).hexdigest()
        if got != want:
            raise SystemExit(f"{path}: sha256 {got}, not the v6.12 file this data comes from")
        texts[key] = raw.decode()
    lists = [reg5(texts[k], n) if k == "table" else reg3(texts[k], n) for n, k in LISTS]
    data = b"".join(struct.pack("<I", len(rows)) for rows in lists)
    for rows in lists:
        data += b"".join(struct.pack("<5I", *r) for r in rows)
    Path(out, "rfk.bin").write_bytes(data)
    for (n, _), rows in zip(LISTS, lists):
        print(f"{n}: {len(rows)}")
    print(f"rfk.bin: sha256 {hashlib.sha256(data).hexdigest()}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2], sys.argv[3])
