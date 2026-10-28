// NONOS Operating System (AGPL-3.0-or-later)
//! The transmit channels the driver uses, by their registers
//! (`rtw89_pci_ch_dma_addr_set`, `pci.h`), and the read-pointer clears.

use super::state::State;

/// One channel: (index, desa_l, number, stop bit, clear bit).
#[derive(Clone, Copy)]
pub struct Ch {
    pub id: u8,
    pub idx: u32,
    pub desa: u32,
    pub num: u32,
    pub stop: u32,
    pub clr: u32,
}

pub const ACH0: Ch = Ch { id: 0, idx: 0x1058, desa: 0x1110, num: 0x1024, stop: 1 << 8, clr: 1 };
pub const CH8: Ch =
    Ch { id: 8, idx: 0x1078, desa: 0x1150, num: 0x1034, stop: 1 << 16, clr: 1 << 8 };
pub const CH12: Ch =
    Ch { id: 12, idx: 0x1080, desa: 0x1160, num: 0x1038, stop: 1 << 18, clr: 1 << 10 };

/// A read-pointer clear: every channel named goes back to 0.
pub fn clear(s: &mut State, a: u32, bits: u32) {
    let named: &[(u32, u32)] = match a {
        0x1014 => &[(ACH0.clr, ACH0.idx), (CH8.clr, CH8.idx), (CH12.clr, CH12.idx)],
        _ => &[(1, 0x1050), (2, 0x1054)],
    };
    for &(_, idx) in named.iter().filter(|n| bits & n.0 != 0) {
        s.regs.insert(idx, 0);
    }
    s.regs.insert(a, 0);
}
