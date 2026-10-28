// NONOS Operating System (AGPL-3.0-or-later)

//! Starting the CPU after the kick, as Linux v6.12
//! `iwl_trans_pcie_gen2_start_fw` does once the context info is handed
//! over. First `iwl_pcie_set_ltr`'s boot-time latency workaround: a discrete
//! AX210 or 22000 part (Linux v6.17 runs the AX210, TY, as discrete,
//! `iwl_ty_mac_cfg`) has its LTR set to about 250 us in CSR_LTR_LONG_VAL_AD,
//! an integrated 22000 part in HPM_UMAC_LTR with HPM_MAC_LTR_CSR enabled;
//! an integrated AX210-family part cannot have it set from the host, so its
//! image-loader cause is cleared and, after the CPU is started, the device
//! kept busy until that cause is set (`iwl_pcie_spin_for_iml`). Then
//! UREG_CPU_INIT_RUN. V2 (BZ): CSR_FUNC_SCRATCH then ROM_START, with nothing
//! for `iwl_pcie_set_ltr` to do.

use super::super::csr::{CsrLayout, CSR_FUNC_SCRATCH, FUNC_SCRATCH_INIT, GP_ROM_START};
use super::super::prph::write_umac;
use super::super::region::Clock;
use super::super::regs::*;
use super::reset::set;
use super::StartError;
use crate::regs::Mmio;

const IML_POLL_MS: u32 = 100;
/// `CSR_LTR_LONG_VAL_AD`.
pub const CSR_LTR_LONG_VAL_AD: usize = 0x0D4;
const LTR_REQ: u32 = 1 << 15;
/// `CSR_LTR_LONG_VAL_AD_SCALE_USEC` in a scale field.
const LTR_SCALE_USEC: u32 = 2 << 10;
const LTR_250: u32 = 250;
/// The 22000 family's integrated LTR registers (peripheral, no UMAC offset).
const HPM_MAC_LTR_CSR: u32 = 0xA0_348C;
const HPM_MAC_LTR_ENABLE_ALL: u32 = 0xF;
const HPM_UMAC_LTR: u32 = 0xA0_3480;
/// The snoop half (bits 0-15) and the no-snoop half (16-31), each 250 us.
pub const LTR_BOOT: u32 =
    (LTR_REQ | LTR_SCALE_USEC | LTR_250) << 16 | LTR_REQ | LTR_SCALE_USEC | LTR_250;

/// Start the CPU; `discrete` is the transport's (not integrated).
pub fn after_kick<M: Mmio, C: Clock>(
    m: &M,
    c: &mut C,
    csr: CsrLayout,
    discrete: bool,
) -> Result<(), StartError> {
    if csr == CsrLayout::V2 {
        m.write32(CSR_FUNC_SCRATCH, FUNC_SCRATCH_INIT);
        set(m, CSR_GP_CNTRL, GP_ROM_START);
        return Ok(());
    }
    let spin = !discrete && csr == CsrLayout::V1;
    let ltr = if discrete {
        m.write32(CSR_LTR_LONG_VAL_AD, LTR_BOOT);
        true
    } else if csr == CsrLayout::Gen2 {
        write_umac(m, c, csr, HPM_MAC_LTR_CSR, HPM_MAC_LTR_ENABLE_ALL)
            && write_umac(m, c, csr, HPM_UMAC_LTR, LTR_BOOT)
    } else {
        m.write32(CSR_MSIX_HW_INT_CAUSES_AD, MSIX_HW_IML);
        true
    };
    if !ltr || !write_umac(m, c, csr, UREG_CPU_INIT_RUN, 1) {
        return Err(StartError::NoNicAccess);
    }
    // `iwl_pcie_spin_for_iml`: back to back, never sleeping (`spin_for`);
    // not fatal, the ALIVE wait decides.
    if spin {
        let _ = c.spin_for(IML_POLL_MS, &mut || {
            let _ = m.read32(CSR_LTR_LAST_MSG);
            m.read32(CSR_MSIX_HW_INT_CAUSES_AD) & MSIX_HW_IML != 0
        });
    }
    Ok(())
}
