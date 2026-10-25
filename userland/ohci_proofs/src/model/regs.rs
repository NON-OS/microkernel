// NONOS Operating System (AGPL-3.0-or-later)
//! The operational registers as the controller answers them: a host reset,
//! the SMM owner flag and its ownership change, list-filled bits the
//! frame runner clears, write-one-to-clear status, and the root hub.

use super::world::{World, PORTS};
use crate::facts::port::*;
use crate::facts::regs::*;

const HCFS_SUSPEND: u32 = 3 << 6;

impl World {
    pub fn read32(&mut self, off: u32) -> u32 {
        match off {
            HC_REVISION => 0x10 | REV_LEGACY,
            HC_CONTROL => self.control | if self.smm { CTL_IR } else { 0 },
            HC_COMMAND_STATUS => self.status,
            HC_INTERRUPT_STATUS => self.int_status,
            HC_HCCA => self.hcca,
            HC_CONTROL_HEAD_ED => self.ctrl_head,
            HC_BULK_HEAD_ED => self.bulk_head,
            HC_FM_INTERVAL => self.fm_interval,
            HC_FM_NUMBER => self.frame as u32,
            HC_RH_DESCRIPTOR_A => PORTS as u32 | 1 << RHA_POTPGT_SHIFT,
            HCE_CONTROL => !self.legacy_off as u32,
            o if (HC_RH_PORT_STATUS..HC_RH_PORT_STATUS + 4 * PORTS as u32).contains(&o) => {
                self.port_read(((o - HC_RH_PORT_STATUS) / 4) as usize)
            }
            _ => 0,
        }
    }

    pub fn write32(&mut self, off: u32, v: u32) {
        match off {
            HC_CONTROL => self.control = v & !CTL_IR,
            HC_COMMAND_STATUS => self.command(v),
            HC_INTERRUPT_STATUS => self.int_status &= !v,
            HC_INTERRUPT_DISABLE => self.int_disabled |= v,
            HC_HCCA => self.hcca = v & !0xFF,
            HC_CONTROL_HEAD_ED => self.ctrl_head = v,
            HC_BULK_HEAD_ED => self.bulk_head = v,
            HC_FM_INTERVAL => self.fm_interval = v,
            HC_PERIODIC_START => self.periodic_start = v,
            HC_RH_STATUS => self.powered |= v & RHS_LPSC != 0,
            HCE_CONTROL => self.legacy_off = v == 0,
            o if (HC_RH_PORT_STATUS..HC_RH_PORT_STATUS + 4 * PORTS as u32).contains(&o) => {
                self.port_write(((o - HC_RH_PORT_STATUS) / 4) as usize, v)
            }
            _ => {}
        }
    }

    fn command(&mut self, v: u32) {
        if v & CS_OCR != 0 && !self.smm_deaf {
            self.smm = false;
        }
        if v & CS_HCR != 0 {
            if self.hcr_stuck {
                self.status |= CS_HCR;
                return;
            }
            // The host controller's registers back to reset, the root hub
            // and memory kept; it comes out of reset suspended.
            (self.control, self.status, self.int_status) = (HCFS_SUSPEND, 0, 0);
            (self.hcca, self.ctrl_head, self.bulk_head) = (0, 0, 0);
            self.fm_interval = FI_DEFAULT;
        }
        self.status |= v & (CS_CLF | CS_BLF);
    }
}
