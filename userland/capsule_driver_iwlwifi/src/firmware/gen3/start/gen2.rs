// NONOS Operating System (AGPL-3.0-or-later)

//! What the 22000 family's start adds, in Linux v6.12's order
//! (`_iwl_trans_pcie_start_hw`): a persistence bit a warm reboot left set is
//! cleared (`iwl_trans_pcie_clear_persistence_bit`, refused when the write
//! protection holds it), and an integrated part's power gating is forced
//! (`gating`). After APM init
//! the radio configuration is written (`nic`).

use super::super::csr::CsrLayout;
use super::super::prph::{read_no_grab, write_no_grab};
use super::super::region::Clock;
use super::{apm_init, prepare_card_hw, sw_reset, StartError};
use crate::regs::Mmio;

mod gating;
mod nic;
pub use nic::set_nic_config;
#[cfg(test)]
pub use nic::{nic_config, NIC_CONFIG_MASK};

const HPM_DEBUG: u32 = 0xA0_3440;
const PERSISTENCE_BIT: u32 = 1 << 12;
const PREG_PRPH_WPROT_22000: u32 = 0xA0_4D00;
const PREG_WFPM_ACCESS: u32 = 1 << 12;
pub(super) const G: CsrLayout = CsrLayout::Gen2;

// A peripheral register as the 22000 family's 20-bit window takes it.
pub(super) const fn w(addr: u32) -> u32 {
    addr & G.prph_mask()
}

/// The 22000 family's `_iwl_trans_pcie_start_hw`. `bisr` is the AX200's
/// BISR workaround (`bisr_workaround`): 2 ms before D0A is asked, 200 us
/// after it comes (`iwl_finish_nic_init`).
pub fn start_hw_gen2<M: Mmio, C: Clock>(
    m: &M,
    c: &mut C,
    integrated: bool,
    bisr: bool,
) -> Result<(), StartError> {
    prepare_card_hw(m, c)?;
    let hpm = read_no_grab(m, w(HPM_DEBUG));
    let error = matches!(hpm & !0xF, 0xA5A5_A5A0 | 0x5A5A_5A50);
    if !error && hpm & PERSISTENCE_BIT != 0 {
        if read_no_grab(m, w(PREG_PRPH_WPROT_22000)) & PREG_WFPM_ACCESS != 0 {
            return Err(StartError::Persistent);
        }
        write_no_grab(m, w(HPM_DEBUG), hpm & !PERSISTENCE_BIT);
    }
    sw_reset(m, c, G);
    prepare_card_hw(m, c)?;
    if integrated {
        gating::force_power_gating(m, c)?;
    }
    bisr_wait(c, bisr, 2000);
    apm_init(m, c, G)?;
    bisr_wait(c, bisr, 200);
    Ok(())
}

/// The BISR workaround's wait, when the part needs it.
pub fn bisr_wait<C: Clock>(c: &mut C, bisr: bool, us: u32) {
    if bisr {
        c.delay_us(us);
    }
}
