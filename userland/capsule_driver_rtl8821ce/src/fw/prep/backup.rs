// NONOS Operating System (AGPL-3.0-or-later)

//! The registers the download borrows, saved and restored.

use super::super::regs::{
    CR_TXDMA_EN, DIS_TSF_UDT, DMA_MAPPING_HIGH_B1, EN_BCN_FUNCTION, FIFOPAGE_INFO_1_DLFW,
    H2CQ_FULL, LD_RQPN, REG_BCN_CTRL, REG_CR, REG_FIFOPAGE_INFO_1, REG_H2CQ_CSR, REG_RQPN_CTRL_2,
    REG_TXDMA_PQ_MAP,
};
use super::kinds::Backup;
use crate::regs::Mmio;

// Save the queue registers the download reprograms, then set them for a
// download that only uses the high-priority (beacon) queue.
pub(super) fn reg_backup<M: Mmio>(mmio: &M) -> Backup {
    let pq_map = mmio.read8(REG_TXDMA_PQ_MAP + 1);
    let cr = mmio.read8(REG_CR);
    let fifopage_info = mmio.read16(REG_FIFOPAGE_INFO_1);
    let rqpn = mmio.read32(REG_RQPN_CTRL_2) | LD_RQPN;
    let bcn_ctrl = mmio.read8(REG_BCN_CTRL);

    mmio.write8(REG_TXDMA_PQ_MAP + 1, DMA_MAPPING_HIGH_B1);
    mmio.write8(REG_CR, CR_TXDMA_EN);
    mmio.write32(REG_H2CQ_CSR, H2CQ_FULL);
    mmio.write16(REG_FIFOPAGE_INFO_1, FIFOPAGE_INFO_1_DLFW);
    mmio.write32(REG_RQPN_CTRL_2, rqpn);
    mmio.write8(REG_BCN_CTRL, (bcn_ctrl & !EN_BCN_FUNCTION) | DIS_TSF_UDT);

    Backup { pq_map, cr, fifopage_info, rqpn, bcn_ctrl }
}

// Put the saved queue registers back after the sections have landed.
pub(super) fn reg_restore<M: Mmio>(mmio: &M, b: Backup) {
    mmio.write8(REG_TXDMA_PQ_MAP + 1, b.pq_map);
    mmio.write8(REG_CR, b.cr);
    mmio.write16(REG_FIFOPAGE_INFO_1, b.fifopage_info);
    mmio.write32(REG_RQPN_CTRL_2, b.rqpn);
    mmio.write8(REG_BCN_CTRL, b.bcn_ctrl);
}
