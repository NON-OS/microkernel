#!/usr/bin/env python3
# NONOS Operating System (AGPL-3.0-or-later)
"""Write the RTL8852B's TX power register images, from Linux v6.12's
rtw89 rtw8852b_table.c (GPL-2.0 OR BSD-3-Clause; taken under BSD-3-Clause,
notice in LICENSE beside this file), for the world-wide regulation
(RTW89_WW) and 20 MHz channels only.

    gen_txpwr.py RTW8852B_TABLE_C OUT_DIR

It computes what rtw89's phy.c writes for a channel:
rtw89_phy_set_txpwr_byrate_ax (11 words from R_AX_PWR_BY_RATE),
rtw89_phy_set_txpwr_offset_ax (one 20-bit word), rtw89_phy_set_txpwr_limit_ax
(two 40-byte pages from R_AX_PWR_LMT) and rtw89_phy_set_txpwr_limit_ru_ax
(two 24-byte pages from R_AX_PWR_RU_LMT). Every table value is shifted
from the RF's quarter-dB to the MAC's half-dB (txpwr_factor_rf 2, mac 1);
a limit is capped at 63, what rtw89's SAR and TPE answer when neither is
set. txpwr.bin holds, as little-endian u32: byrate words for 2 GHz, then
5 GHz (11 each); the offset word for each band; then, per channel index
(2 GHz 1..14, then rtw89_channel_to_idx's 53 5 GHz slots), 20 limit words
and 12 RU limit words.
"""

import hashlib
import re
import struct
import sys
from pathlib import Path

SOURCE_SHA256 = "8b9b5fc32628037d8dec86cff976913164723d87abe38fd09d229c57b509b42f"
WW = "RTW89_WW"
CCK, OFDM, MCS, HEDCM, OFFSET = range(5)
IDX_NUM = {CCK: 4, OFDM: 8, MCS: 12, HEDCM: 4, OFFSET: 5}
NSS_NUM = {CCK: 1, OFDM: 1, MCS: 4, HEDCM: 2, OFFSET: 1}


def s8(v):
    v &= 0xFF
    return v - 256 if v >= 128 else v


def rf_to_mac(v):
    return s8(v) >> 1


def body(text, name):
    m = re.search(re.escape(name) + r"\[[^=]*=\s*\{(.*?)\n\};", text, re.S)
    if not m:
        raise SystemExit(f"{name}: not found")
    return m.group(1)


def byrate(text):
    rows = re.findall(r"\{\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(0x[0-9a-fA-F]+),\s*\}",
                      body(text, "rtw89_8852b_txpwr_byrate"))
    byr = {}
    for band, nss, rs, shf, ln, data in rows:
        data = int(data, 16)
        for i in range(int(ln)):
            byr[(int(band), int(nss), int(rs), int(shf) + i)] = (data >> (8 * i)) & 0xFF
    return byr


def designated(text, name):
    """Only the RTW89_WW entries: (indices before and after the regd) -> value."""
    entry = r"((?:\[\w+\])+)\s*=\s*(-?\d+),"
    out = {}
    for idx, val in re.findall(entry, body(text, name)):
        parts = re.findall(r"\[(\w+)\]", idx)
        if WW in parts:
            out[tuple(int(p) for p in parts if p != WW)] = int(val)
    return out


def read_byr(byr, band, nss, rs, idx):
    if rs == CCK:
        band = 0
    return rf_to_mac(byr.get((band, nss, rs, idx), 0))


def byrate_words(byr, band):
    words = []
    for nss in range(2):
        for rs in (CCK, OFDM, MCS, HEDCM):
            if nss >= NSS_NUM[rs]:
                continue
            v = [read_byr(byr, band, nss, rs, i) for i in range(IDX_NUM[rs])]
            for q in range(0, len(v), 4):
                words.append(sum((v[q + k] & 0xFF) << (8 * k) for k in range(4)))
    return words


def offset_word(byr, band):
    v = [read_byr(byr, band, 0, OFFSET, i) for i in range(5)]
    return sum((v[i] & 0xF) << (4 * i) for i in range(5))


def pack(page):
    return [sum((page[q + k] & 0xFF) << (8 * k) for k in range(4)) for q in range(0, len(page), 4)]


def limit(tbl, bw, ntx, rs, bf, ch_idx):
    return min(rf_to_mac(tbl.get((bw, ntx, rs, bf, ch_idx), 0)), 63)


def ch_words(lmt, ru, ch_idx):
    words = []
    for ntx in range(2):
        page = [0] * 40
        for bf in range(2):
            page[0 + bf] = limit(lmt, 0, ntx, CCK, bf, ch_idx)
            page[2 + bf] = limit(lmt, 1, ntx, CCK, bf, ch_idx)
            page[4 + bf] = limit(lmt, 0, ntx, OFDM, bf, ch_idx)
            page[6 + bf] = limit(lmt, 0, ntx, MCS, bf, ch_idx)
        words += pack(page)
    for ntx in range(2):
        page = [0] * 24
        for k in range(3):  # RU26, RU52, RU106: section 0 only at 20 MHz
            page[8 * k] = min(rf_to_mac(ru.get((k, ntx, ch_idx), 0)), 63)
        words += pack(page)
    return words


def main(src, out):
    raw = Path(src).read_bytes()
    if hashlib.sha256(raw).hexdigest() != SOURCE_SHA256:
        raise SystemExit("rtw8852b_table.c is not the v6.12 file this data comes from")
    text = raw.decode()
    byr = byrate(text)
    words = byrate_words(byr, 0) + byrate_words(byr, 1)
    words += [offset_word(byr, 0), offset_word(byr, 1)]
    l2, l5 = designated(text, "rtw89_8852b_txpwr_lmt_2g"), designated(text, "rtw89_8852b_txpwr_lmt_5g")
    r2, r5 = designated(text, "rtw89_8852b_txpwr_lmt_ru_2g"), designated(text, "rtw89_8852b_txpwr_lmt_ru_5g")
    for i in range(14):
        words += ch_words(l2, r2, i)
    for i in range(53):
        words += ch_words(l5, r5, i)
    data = b"".join(struct.pack("<I", w) for w in words)
    Path(out, "txpwr.bin").write_bytes(data)
    print(f"txpwr.bin: {len(words)} words, sha256 {hashlib.sha256(data).hexdigest()}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
