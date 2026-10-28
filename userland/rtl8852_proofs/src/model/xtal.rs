// NONOS Operating System (AGPL-3.0-or-later)
//! The XTAL SI bus (`R_AX_WLAN_XTAL_SI_CTRL`), the DAV efuse behind it,
//! the main efuse (`R_AX_EFUSE_CTRL`) and the LTE coexistence block's
//! indirect registers: each takes a command word and answers in place.

use super::state::State;

const CMD_POLL: u32 = 0x8000_0000;
const DAV_CTRL: u8 = 0x63;
const DAV_LOW: u8 = 0x62;
const DAV_VAL: u8 = 0x7a;
const SI_RDY: u8 = 0x20;

pub fn si(s: &mut State, cmd: u32) {
    if cmd & CMD_POLL == 0 {
        s.violations.push(format!("xtal si command {cmd:#x} without the poll bit"));
        return;
    }
    let (addr, data, mask) = (cmd as u8 as usize, (cmd >> 8) as u8, (cmd >> 16) as u8);
    let mut out = cmd & !CMD_POLL;
    match (cmd >> 24) & 0x3 {
        0 => {
            s.xtal[addr] = (s.xtal[addr] & !mask) | (data & mask);
            if addr == DAV_CTRL as usize && mask == 0xc0 && data == 0 {
                dav_read(s);
            }
        }
        1 => out = (out & !0xff00) | (s.xtal[addr] as u32) << 8,
        m => s.violations.push(format!("xtal si mode {m}")),
    }
    s.regs.insert(0x270, out);
}

/// The DAV cell named by the address bytes, into the read-value byte.
fn dav_read(s: &mut State) {
    if s.faults.efuse_dead {
        return;
    }
    let a = s.xtal[DAV_LOW as usize] as usize | ((s.xtal[DAV_CTRL as usize] & 7) as usize) << 8;
    s.xtal[DAV_VAL as usize] = s.dav.get(a).copied().unwrap_or(0xff);
    s.xtal[DAV_CTRL as usize] |= SI_RDY;
}

/// A main efuse read: the cell and the ready bit, unless the efuse is dead.
pub fn efuse(s: &mut State, ctl: u32) {
    if s.faults.efuse_dead {
        return;
    }
    let a = ((ctl >> 16) & 0x7ff) as usize;
    let cell = s.efuse.get(a).copied().unwrap_or(0xff) as u32;
    s.regs.insert(0x30, (ctl & !0xff) | 1 << 29 | cell);
}

pub fn lte(s: &mut State, cmd: u32) {
    let off = cmd & 0xffff;
    if cmd & 0x4000_0000 != 0 {
        let v = s.word(0xdaf4);
        s.lte.insert(off, v);
    } else {
        let v = s.lte.get(&off).copied().unwrap_or(0);
        s.regs.insert(0xdaf8, v);
    }
}
