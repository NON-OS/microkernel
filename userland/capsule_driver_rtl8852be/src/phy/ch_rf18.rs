// NONOS Operating System (AGPL-3.0-or-later)

//! RF 0x18's channel and bandwidth fields as `_ch_setting` and
//! `_bw_setting` (Linux v6.12 rtw89 `rtw8852b_rfk.c`) rewrite them: the
//! channel number, the 5 GHz band bits, the 20 MHz bandwidth code, with
//! the lock, TRX, beacon and BW2 bits cleared and BW2 set again.

use super::rf::RFREG_MASK;

const CFGCH_BW: u32 = 0xc00;
const CFGCH_LOW: u32 = 0x8000 | 0x4000 | 0x2000 | 0x1000; // POW_LCK|TRX_AH|BCN|BW2
const CFGCH_BW2: u32 = 0x1000;
const CFGCH_BAND: u32 = 0x3_0000 | 0x300;

/// The new RF 0x18 value `_ch_setting` derives from `old`.
pub const fn reg18_ch(old: u32, ch: u8) -> u32 {
    let mut v = old & !(CFGCH_BAND | CFGCH_LOW | 0xff);
    v |= ch as u32;
    if ch > 14 {
        v |= 0x1_0000 | 0x100; // BAND1 and BAND0 at 5G
    }
    (v & !(CFGCH_LOW & RFREG_MASK)) | CFGCH_BW2
}

/// The new RF 0x18 value `_bw_setting` derives from `old` at 20 MHz.
pub const fn reg18_bw20(old: u32) -> u32 {
    let v = (old & !CFGCH_BW) | (3 << 10);
    (v & !(CFGCH_LOW & RFREG_MASK)) | CFGCH_BW2
}
