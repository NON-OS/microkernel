// NONOS Operating System (AGPL-3.0-or-later)
//! The PCIe configuration space as the DBI window reaches it
//! (`rtw89_dbi_read8`, `_write8`): a read flag puts the dword at the
//! address in `R_AX_DBI_RDATA`, a write flag stores the bytes the enable
//! nibble names from `R_AX_DBI_WDATA`; the flag byte clears when done.

use super::state::State;

const FLAG: u32 = 0x1090;
const WDATA: u32 = 0x1094;
const RDATA: u32 = 0x1098;

pub fn take(s: &mut State, v: u32) {
    let addr = (v & 0xffc) as u16;
    match (v >> 16) & 0xff {
        0x2 => {
            let b: [u8; 4] = core::array::from_fn(|i| cfg(s, addr + i as u16));
            s.regs.insert(RDATA, u32::from_le_bytes(b));
        }
        0x1 => {
            let data = s.word(WDATA).to_le_bytes();
            for i in (0..4).filter(|i| (v >> 12) & (1 << i) != 0) {
                s.cfg.insert(addr + i as u16, data[i]);
            }
        }
        0 => return,
        f => s.violations.push(format!("dbi flag {f:#x}")),
    }
    s.regs.insert(FLAG, v & !0xff_0000);
}

pub fn cfg(s: &State, a: u16) -> u8 {
    s.cfg.get(&a).copied().unwrap_or(0)
}
