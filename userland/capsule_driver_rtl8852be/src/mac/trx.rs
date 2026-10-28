// NONOS Operating System (AGPL-3.0-or-later)

//! `trx_init_ax` (Linux v6.12 rtw89 `mac.c`) on the 8852B's single-band
//! plan: the DMAC, CMAC 0, the error masks, and the release reports to the
//! host (`set_host_rpr_ax`, PCIe "POH" mode: every report kind, 30 per
//! batch, flushed after 255 units). The plan has no CMAC1 quota, so no
//! DBCC and no second band.

use super::{cmac_a::cmac_a, cmac_b::cmac_b, dmac::dmac_init, imr, MacFail};
use crate::regs::{Mmio, Rw};

pub const R_AX_WDRLS_CFG: u32 = 0x9408;
pub const B_AX_WDRLS_MODE_MASK: u32 = 0x3;
pub const RPR_MODE_POH: u32 = 0;
pub const R_AX_RLSRPT0_CFG0: u32 = 0x9410;
pub const B_AX_RLSRPT0_FLTR_MAP_MASK: u32 = 0xf00_0000;
pub const R_AX_RLSRPT0_CFG1: u32 = 0x9414;
pub const B_AX_RLSRPT0_AGGNUM_MASK: u32 = 0xff;
pub const B_AX_RLSRPT0_TO_MASK: u32 = 0xff_0000;

pub fn trx_init(r: &impl Mmio) -> Result<(), MacFail> {
    dmac_init(r)?;
    cmac_a(r)?;
    cmac_b(r)?;
    imr::enable(r);
    r.mask32(R_AX_WDRLS_CFG, B_AX_WDRLS_MODE_MASK, RPR_MODE_POH);
    r.set32(R_AX_RLSRPT0_CFG0, B_AX_RLSRPT0_FLTR_MAP_MASK);
    r.mask32(R_AX_RLSRPT0_CFG1, B_AX_RLSRPT0_AGGNUM_MASK, 30);
    r.mask32(R_AX_RLSRPT0_CFG1, B_AX_RLSRPT0_TO_MASK, 255);
    Ok(())
}
