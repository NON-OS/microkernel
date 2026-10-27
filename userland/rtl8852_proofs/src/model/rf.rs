// NONOS Operating System (AGPL-3.0-or-later)
//! The RF registers behind the serial interface (SWSI, `rtw89_phy_write_rf_v1`
//! and `_read_rf_v1`): a write word names path, address and 20 bits of data,
//! masked by `R_SWSI_BIT_MASK_V1` when its top bit says so; a read address
//! puts the register's value under `R_SWSI_V1` with the done bit. The model
//! completes each at once, so the busy bits never read set.

use super::state::State;

const BB: u32 = 0x1_0000;
const MASK_REG: u32 = BB + 0x374;
const SWSI_V1: u32 = BB + 0x174c;
const R_DONE: u32 = 0x400_0000;
const RF: u32 = 0xf_ffff;

fn path(s: &mut State, p: u32) -> Option<usize> {
    if p > 1 {
        s.violations.push(format!("rf path {p}"));
        return None;
    }
    Some(p as usize)
}

pub fn swsi_write(s: &mut State, v: u32) {
    let Some(p) = path(s, (v >> 28) & 0x7) else { return };
    let addr = ((v >> 20) & 0xff) as usize;
    let data = v & RF;
    let mask = if v & 0x8000_0000 != 0 { s.word(MASK_REG) & RF } else { RF };
    s.rf[p][addr] = (s.rf[p][addr] & !mask) | (data & mask);
}

pub fn swsi_read(s: &mut State, v: u32) {
    let Some(p) = path(s, (v >> 8) & 0x7) else { return };
    let val = s.rf[p][(v & 0xff) as usize];
    s.regs.insert(SWSI_V1, R_DONE | val);
}

/// The channel RF path A is tuned to: (5 GHz, channel), from RR_CFGCH.
pub fn tuned(s: &State) -> Option<(bool, u8)> {
    let v = s.rf[0][0x18];
    let ch = (v & 0xff) as u8;
    (ch != 0).then_some((v & 0x100 != 0, ch))
}
