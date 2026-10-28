# Generated tables

The driver's register tables, generated from Linux v6.12 rtw89 sources
under their BSD-3-Clause option (notice: `LICENSE` here). Each generator
reads one rtw89 file, checks its sha256 first, and writes little-endian
`u32` words; rebuild them only from the same sources.

| File | Generator | Source | Read by |
|---|---|---|---|
| `bb.bin` | `gen.py` | `rtw8852b_table.c`, `rtw89_8852b_phy_bb_regs` | `phy/init_bb.rs` |
| `bb_gain.bin` | `gen.py` | `rtw89_8852b_phy_bb_reg_gain` | `phy/init_bb.rs` |
| `radioa.bin`, `radiob.bin` | `gen.py` | `rtw89_8852b_phy_radio{a,b}_regs` | `phy/init_rf.rs` |
| `nctl.bin` | `gen.py` | `rtw89_8852b_phy_nctl_regs` | `phy/nctl.rs` |
| `txpwr.bin` | `gen_txpwr.py` | the by-rate, offset, limit and RU limit tables, world-wide regulation | `phy/txpwr.rs` |
| `rfk.bin` | `gen_rfk.py` | `rtw8852b_rfk_table.c` lists and `rtw8852b_rfk.c` | `rfk/reg5.rs` |

Layouts:

- `bb.bin`, `radio*.bin`, `nctl.bin`: (address, data) word pairs, rtw89's
  conditional headers kept as they are (`phy/headline.rs` evaluates them
  for the board's RFE type and cut).
- `txpwr.bin` (2168 words): by rate, 11 words per band; then one offset
  word per band; then per channel index (14 at 2.4 GHz, 53 at 5 GHz) 20
  limit words and 12 RU limit words.
- `rfk.bin`: a header of 13 list counts, then 5-word entries (flag, path,
  address, mask, data).
