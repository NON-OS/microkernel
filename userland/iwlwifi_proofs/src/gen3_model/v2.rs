// NONOS Operating System (AGPL-3.0-or-later)

//! The modeled device's boot ROM, and its V2 (BZ) CSR layout. A V1 device
//! starts on UREG_CPU_INIT_RUN; a V2 one only on ROM_START with
//! CSR_FUNC_SCRATCH set first, wakes only on MAC_INIT or its own access
//! request (MAC_STATUS, never the V1 clock bit), acknowledges the bus-master
//! stop in CSR_GP_CNTRL, and takes the receive free index through
//! HBUS_TARG_WRPTR at queue 512. Nothing V1 answers on a V2 device.

use super::{Model, Out};
use crate::gen3::csr::{CSR_FUNC_SCRATCH, FUNC_SCRATCH_INIT, GP_BUS_MASTER_DISABLE_REQ};
use crate::gen3::csr::{GP_BUS_MASTER_DISABLE_STATUS, GP_BZ_MAC_ACCESS_REQ, GP_MAC_INIT};
use crate::gen3::csr::{GP_MAC_STATUS, GP_ROM_START};
use crate::gen3::regs::{CSR_GP_CNTRL, CSR_INT, HBUS_TARG_WRPTR, INT_BIT_ALIVE};

/// CSR_GP_CNTRL as a V2 device shows it.
pub(super) fn gp_read(v: u32) -> u32 {
    if v & (GP_MAC_INIT | GP_BZ_MAC_ACCESS_REQ) != 0 {
        v | GP_MAC_STATUS
    } else {
        v & !GP_MAC_STATUS
    }
}

impl Model {
    /// The ROM ran the image: the ALIVE cause and notification.
    pub(super) fn raise_alive(&self) {
        let (no_alive, status, sku) = {
            let s = self.s.borrow();
            (s.no_alive, s.alive_status, s.alive_sku)
        };
        if no_alive {
            return;
        }
        let mut s = self.s.borrow_mut();
        *s.regs.entry(CSR_INT).or_insert(0) |= INT_BIT_ALIVE;
        let mut p = vec![0u8; 144];
        p[0..2].copy_from_slice(&status.to_le_bytes());
        p[100..104].copy_from_slice(&0x4Au32.to_le_bytes());
        for (i, w) in sku.iter().enumerate() {
            p[116 + 4 * i..120 + 4 * i].copy_from_slice(&w.to_le_bytes());
        }
        s.queued.push(Out { cmd: 0x01, group: 0, seq: 0x8000, payload: p });
    }

    /// A V2 device's own handling of a write; `false` leaves it to V1's.
    pub(super) fn v2_write(&self, off: usize, val: u32) -> bool {
        if off == HBUS_TARG_WRPTR {
            if val >> 16 != 512 {
                return false;
            }
            self.s.borrow_mut().widx = (val & 0xFFFF) as usize;
            self.deliver();
            return true;
        }
        let mut v = val;
        if v & GP_BUS_MASTER_DISABLE_REQ != 0 {
            v |= GP_BUS_MASTER_DISABLE_STATUS;
        }
        let scratch = self.reg(CSR_FUNC_SCRATCH) == FUNC_SCRATCH_INIT;
        let was = self.reg(CSR_GP_CNTRL);
        self.s.borrow_mut().regs.insert(CSR_GP_CNTRL, v);
        if v & GP_ROM_START != 0 && was & GP_ROM_START == 0 && scratch {
            self.raise_alive();
        }
        true
    }
}
