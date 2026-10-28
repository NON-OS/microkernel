// NONOS Operating System (AGPL-3.0-or-later)

//! The modeled device as a 22000-family part (`State::gen2`): the context
//! info's address taken from CSR_CTXT_INFO_BA, peripheral addresses through
//! a 20-bit window with no UMAC offset (UREG_CPU_INIT_RUN raises ALIVE only
//! once the context info was given), a free-ring entry that is the buffer's
//! address with its id in the low 12 bits, four-byte used-ring entries,
//! several packets to a buffer when `pack` asks for it (each on a 64-byte
//! boundary, then the end marker), and received frames under the version 1
//! descriptor, 16 bytes shorter than the version 3 one the scripts write.

use super::{Mem, Model, Out};
use crate::gen3::plan::RX_FREE;
use crate::gen3::region::Region;
use crate::gen3::regs::{HBUS_TARG_PRPH_RADDR, HBUS_TARG_PRPH_RDAT};
use crate::gen3::regs::{UREG_CPU_INIT_RUN, UREG_DOORBELL_TO_ISR6, UREG_DOORBELL_TO_ISR6_PNVM};

pub const CSR_CTXT_INFO_BA: usize = 0x40;
const W: u32 = 0x000F_FFFF;

/// Free-ring entry `i`: the buffer id and its offset in the buffer region.
pub(super) fn free_entry(ctrl: &Mem, rbs: &Mem, i: usize, gen2: bool) -> (u16, usize) {
    let mut d = [0u8; 16];
    if gen2 {
        ctrl.read(RX_FREE + i * 8, &mut d[..8]);
        let e = u64::from_le_bytes(d[..8].try_into().unwrap());
        return ((e & 0xFFF) as u16, ((e & !0xFFF) - rbs.dev) as usize);
    }
    ctrl.read(RX_FREE + i * 16, &mut d);
    let addr = u64::from_le_bytes(d[8..16].try_into().unwrap());
    (u16::from_le_bytes([d[0], d[1]]), (addr - rbs.dev) as usize)
}

mod rb;
pub(super) use rb::{packets, put_used};

impl Model {
    /// A 22000-family part's own handling of a write; `false` leaves it to
    /// the V1 model's.
    pub(super) fn gen2_write(&self, off: usize, val: u32) -> bool {
        let mut s = self.s.borrow_mut();
        match off {
            CSR_CTXT_INFO_BA => s.ctxt_ba = (s.ctxt_ba & !0xFFFF_FFFF) | u64::from(val),
            0x44 => s.ctxt_ba = (s.ctxt_ba & 0xFFFF_FFFF) | u64::from(val) << 32,
            HBUS_TARG_PRPH_RADDR => {
                let v = s.prph_vals.get(&(val & W)).copied().unwrap_or(0);
                s.regs.insert(HBUS_TARG_PRPH_RDAT, v);
            }
            _ => return false,
        }
        s.regs.insert(off, val);
        true
    }

    /// A 22000-family part's peripheral write (already in the 20-bit window):
    /// it reads back what was written.
    pub(super) fn gen2_prph(&self, addr: u32, val: u32) {
        self.s.borrow_mut().prph_vals.insert(addr, val);
        if addr == UREG_CPU_INIT_RUN & W && val == 1 && self.s.borrow().ctxt_ba != 0 {
            self.raise_alive();
        }
        if addr == UREG_DOORBELL_TO_ISR6 & W && val & UREG_DOORBELL_TO_ISR6_PNVM != 0 {
            self.queue(Out { cmd: 0xFE, group: 0x0C, seq: 0x8000, payload: vec![0; 4] });
        }
    }
}
