// NONOS Operating System (AGPL-3.0-or-later)

//! The error interrupt masks `trx_init_ax` turns on (Linux v6.12 rtw89
//! `mac.c`: `enable_imr_ax` for the DMAC then CMAC 0, with the 8852B's
//! `rtw8852b_imr_info`, then `err_imr_ctrl_ax`). They route the MAC's
//! error reports to the firmware, which recovers what it can alone. A row
//! clears (when its rtw89 helper clears) and then sets.

use crate::regs::{Mmio, Rw};

/// (register, bits cleared, bits set), in rtw89's order.
pub const IMR: [(u32, Option<u32>, u32); 24] = [
    (0x9430, Some(0x0000_3337), 0x0000_3327), // WDRLS_ERR_IMR
    (0x9d1c, None, 0x0000_0008),              // SEC_DEBUG
    (0x9bf4, Some(0x0000_003e), 0x0000_0000), // MPDU_TX_ERR_IMR
    (0x9cf4, Some(0x0000_000b), 0x0000_0000), // MPDU_RX_ERR_IMR
    (0x9ef0, Some(0x0000_0007), 0x0000_0007), // STA_SCHEDULER_ERR_IMR
    (0x9f1c, Some(0x0000_030f), 0x0000_0101), // TXPKTCTL_ERR_IMR_ISR
    (0x9f2c, Some(0x0000_030f), 0x0000_0303), // TXPKTCTL_ERR_IMR_ISR_B1
    (0x8c38, Some(0x070f_f0ff), 0x070f_f0ff), // WDE_ERR_IMR
    (0x9038, Some(0x070f_f0ff), 0x070f_f0df), // PLE_ERR_IMR
    (0x9a20, None, 0x0000_0001),              // PKTIN_ERR_IMR
    (0x8850, Some(0xff0f_ffff), 0x0c00_0161), // HOST_DISPATCHER_ERR_IMR
    (0x8854, Some(0xff07_ffff), 0x0400_0062), // CPU_DISPATCHER_ERR_IMR
    (0x8858, Some(0x3f03_1f1f), 0x0000_0000), // OTHER_DISPATCHER_ERR_IMR
    (0x9840, Some(0x0000_1111), 0x0000_1111), // CPUIO_ERR_IMR
    (0x960c, None, 0x0000_0001),              // BBRPT_COM_ERR_IMR_ISR
    (0x962c, Some(0x0000_00ff), 0x0000_0000), // BBRPT_CHINFO_ERR_IMR_ISR
    (0x963c, None, 0x0000_0001),              // BBRPT_DFS_ERR_IMR_ISR
    (0x966c, None, 0x0000_0001),              // LA_ERRFLAG
    (0xc3e8, Some(0x0000_0003), 0x0000_0001), // SCHEDULE_ERR_IMR
    (0xc6c0, Some(0xffff_ffff), 0x1080_0001), // PTCL_IMR0
    (0xc800, Some(0x0080_c000), 0x0000_c000), // DLE_CTRL (CDMA)
    (0xccfc, Some(0x0000_0000), 0x0000_0000), // PHYINFO_ERR_IMR
    (0xcef4, Some(0x000f_f000), 0x000e_4000), // RMAC_ERR_ISR
    (0xccec, Some(0x0000_0780), 0x0000_0780), // TMAC_ERR_IMR_ISR
];
pub const R_AX_DMAC_ERR_IMR: u32 = 0x8520;
pub const R_AX_CMAC_ERR_IMR: u32 = 0xc160;

pub fn enable(r: &impl Mmio) {
    for (reg, clr, set) in IMR {
        if let Some(c) = clr {
            r.clr32(reg, c);
        }
        r.set32(reg, set);
    }
    r.write32(R_AX_DMAC_ERR_IMR, u32::MAX);
    r.write32(R_AX_CMAC_ERR_IMR, u32::MAX);
}
