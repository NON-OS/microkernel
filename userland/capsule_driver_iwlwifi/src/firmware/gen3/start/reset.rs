// NONOS Operating System (AGPL-3.0-or-later)

//! Resetting and quieting the device, in the family's CSR layout: the
//! software reset (`iwl_trans_pcie_sw_reset`), the bus-master stop
//! (`iwl_pcie_apm_stop_master`) and the interrupt masks.

use super::super::csr::GP_BUS_MASTER_DISABLE_STATUS as STOPPED;
use super::super::csr::{CsrLayout, GP_BUS_MASTER_DISABLE_REQ as STOP, GP_SW_RESET};
use super::super::region::Clock;
use super::super::regs::*;
use crate::regs::Mmio;

const STOP_MASTER_MS: u32 = 1;

pub(super) fn set<M: Mmio>(m: &M, reg: usize, bits: u32) {
    m.write32(reg, m.read32(reg) | bits);
}

/// The software reset, then its settle time (5-6 ms on V1, 10-20 ms on V2).
pub fn sw_reset<M: Mmio, C: Clock>(m: &M, c: &mut C, csr: CsrLayout) {
    let (reg, bit, settle_us) = match csr {
        CsrLayout::V1 | CsrLayout::Gen2 => (CSR_RESET, RESET_SW_RESET, 6000),
        CsrLayout::V2 => (CSR_GP_CNTRL, GP_SW_RESET, 20_000),
    };
    set(m, reg, bit);
    c.delay_us(settle_us);
}

/// Stop the device's bus mastering and wait to see it stopped (100 us in
/// Linux; a millisecond here). V2 then settles 10-20 ms, as Linux does.
pub fn stop_master<M: Mmio, C: Clock>(m: &M, c: &mut C, csr: CsrLayout) -> bool {
    match csr {
        CsrLayout::V1 | CsrLayout::Gen2 => {
            set(m, CSR_RESET, RESET_STOP_MASTER);
            c.poll_for(STOP_MASTER_MS, &mut || m.read32(CSR_RESET) & RESET_MASTER_DISABLED != 0)
        }
        CsrLayout::V2 => {
            set(m, CSR_GP_CNTRL, STOP);
            let stopped = c.poll_for(STOP_MASTER_MS, &mut || m.read32(CSR_GP_CNTRL) & STOPPED != 0);
            c.delay_us(20_000);
            stopped
        }
    }
}

pub(super) fn mask_all<M: Mmio>(m: &M) {
    m.write32(CSR_INT_MASK, 0);
    m.write32(CSR_INT, 0xFFFF_FFFF);
    m.write32(CSR_FH_INT_STATUS, 0xFFFF_FFFF);
    m.write32(CSR_MSIX_FH_INT_MASK_AD, 0xFFFF_FFFF);
    m.write32(CSR_MSIX_HW_INT_MASK_AD, 0xFFFF_FFFF);
}

/// Stop the device signalling interrupts once ALIVE is in: the driver polls
/// the receive index and the cause registers, which still latch with every
/// mask set, and an INTx line left asserted could be shared with another
/// device. The cause bits are left for the error checks to read.
pub fn mask_interrupts<M: Mmio>(m: &M) {
    m.write32(CSR_INT_MASK, 0);
    m.write32(CSR_MSIX_FH_INT_MASK_AD, 0xFFFF_FFFF);
    m.write32(CSR_MSIX_HW_INT_MASK_AD, 0xFFFF_FFFF);
}

/// What a failed bring-up leaves the card at (`iwl_trans_pcie_gen2_stop_device`
/// reduced to the part that quiets it): every interrupt masked, bus mastering
/// stopped (a timeout is not fatal, as in Linux), then a software reset that
/// halts the firmware. `false` when the master never reported stopped.
pub fn stop_device<M: Mmio, C: Clock>(m: &M, c: &mut C, csr: CsrLayout) -> bool {
    mask_all(m);
    let stopped = stop_master(m, c, csr);
    sw_reset(m, c, csr);
    stopped
}
