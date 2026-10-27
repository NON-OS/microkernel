// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8852b_set_channel_rf` (Linux v6.12 rtw89 `rtw8852b_rfk.c`) at 20
//! MHz: RF 0x18 (`RR_CFGCH`, and its direct copy `RR_CFGCH_V1`) on both
//! paths gets the channel, band and bandwidth; path A's write goes through
//! `_set_s0_arfc18`, which waits up to 1 ms for the synthesiser's lock
//! calibration, then `_lck_check` repairs a synthesiser that did not lock.
//! Then the RX baseband filter is set for 20 MHz (`_set_rxbb_bw`).

use super::bb::Bb;
use super::ch_rf18::{reg18_bw20, reg18_ch};
use super::rf::{INV, RFREG_MASK};
use crate::regs::Mmio;
use crate::wait::until_us;

pub const RR_CFGCH: u32 = 0x18;
pub const RR_CFGCH_V1: u32 = 0x10018;
const RR_LDO: u32 = 0xb1;
const RR_LPF: u32 = 0xb7;
const RR_LCKST: u32 = 0xcf;

/// `_set_s0_arfc18`; true when the lock calibration timed out.
fn s0_arfc18(bb: &Bb<'_, impl Mmio>, val: u32) -> bool {
    let bak = bb.read_rf(0, RR_LDO, RFREG_MASK);
    bb.write_rf(0, RR_LDO, 0x1c0, 1);
    bb.write_rf(0, RR_CFGCH, RFREG_MASK, val);
    let ok = until_us(1000, || bb.read_rf(0, RR_LPF, 0x100) == 0);
    bb.write_rf(0, RR_LDO, RFREG_MASK, bak);
    !ok
}

pub fn set_channel_rf(r: &impl Mmio, ch: u8) {
    let bb = Bb(r);
    for addr in [RR_CFGCH, RR_CFGCH_V1] {
        for path in 0..2u8 {
            let v = reg18_ch(bb.read_rf(path, addr, RFREG_MASK), ch);
            if path == 0 && addr == RR_CFGCH {
                if !s0_arfc18(&bb, v) {
                    super::lck::lck_check(&bb, |b, v| s0_arfc18(b, v));
                }
            } else {
                bb.write_rf(path, addr, RFREG_MASK, v);
            }
            bb.write_rf(path, RR_LCKST, 0x1, 0);
            bb.write_rf(path, RR_LCKST, 0x1, 1);
        }
    }
    for addr in [RR_CFGCH, RR_CFGCH_V1] {
        for path in 0..2u8 {
            let old = bb.read_rf(path, addr, RFREG_MASK);
            if old != INV {
                bb.write_rf(path, addr, RFREG_MASK, reg18_bw20(old));
            }
        }
    }
    for path in 0..2u8 {
        bb.write_rf(path, 0xee, 0x4, 1); // RR_LUTWE2 RTXBW
        bb.write_rf(path, 0x33, 0x1f, 0x12); // RR_LUTWA M2
        bb.write_rf(path, 0x3f, 0x3f, 0x1b); // RR_LUTWD0 LB, 20 MHz
        bb.write_rf(path, 0xee, 0x4, 0);
    }
}
