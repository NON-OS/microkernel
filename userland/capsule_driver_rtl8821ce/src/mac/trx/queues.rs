// NONOS Operating System (AGPL-3.0-or-later)

//! The priority queues' page counts and the H2C queue.

use super::bounds::{
    H2CQ_ADDR, H2CQ_SIZE, H2CQ_TAIL, H2C_ADDR_KEEP, PUBQ_NUM, RSVD_BOUNDARY, RXFF_BNDY,
};
use super::engines::poll_llt;
use super::pages::{
    AUTO_INIT_LLT_V1, DROP_DATA_EN_B1, EN_WR_FREE_TAIL_B2, EXQ_NUM, HQ_NUM, LQ_NUM, NQ_NUM,
};
use super::regs::{
    LD_RQPN, REG_AUTO_LLT_V1, REG_BCNQ1_BDNY_V1, REG_BCNQ_BDNY_V1, REG_CR, REG_FIFOPAGE_CTRL_2,
    REG_FIFOPAGE_INFO_1, REG_FIFOPAGE_INFO_2, REG_FIFOPAGE_INFO_3, REG_FIFOPAGE_INFO_4,
    REG_FIFOPAGE_INFO_5, REG_FWHW_TXQ_CTRL, REG_H2C_HEAD, REG_H2C_INFO, REG_H2C_PKT_READADDR,
    REG_H2C_PKT_WRITEADDR, REG_H2C_READ_ADDR, REG_H2C_TAIL, REG_RQPN_CTRL_2, REG_RXFF_BNDY,
    REG_TXDMA_OFFSET_CHK,
};
use super::reset::{rmw32, rmw8, set32, set8};
use crate::regs::Mmio;

// priority_queue_cfg: load the per-queue page counts, latch them, set the
// reserved-page boundaries and the receive FIFO boundary, then run the automatic
// link-list-table init and wait for it to finish.
pub(super) fn priority_queue_cfg<M: Mmio>(mmio: &M) -> bool {
    mmio.write16(REG_FIFOPAGE_INFO_1, HQ_NUM);
    mmio.write16(REG_FIFOPAGE_INFO_2, LQ_NUM);
    mmio.write16(REG_FIFOPAGE_INFO_3, NQ_NUM);
    mmio.write16(REG_FIFOPAGE_INFO_4, EXQ_NUM);
    mmio.write16(REG_FIFOPAGE_INFO_5, PUBQ_NUM);
    set32(mmio, REG_RQPN_CTRL_2, LD_RQPN);
    mmio.write16(REG_FIFOPAGE_CTRL_2, RSVD_BOUNDARY);
    set8(mmio, REG_FWHW_TXQ_CTRL + 2, EN_WR_FREE_TAIL_B2);
    mmio.write16(REG_BCNQ_BDNY_V1, RSVD_BOUNDARY);
    mmio.write16(REG_FIFOPAGE_CTRL_2 + 2, RSVD_BOUNDARY);
    mmio.write16(REG_BCNQ1_BDNY_V1, RSVD_BOUNDARY);
    mmio.write32(REG_RXFF_BNDY, RXFF_BNDY);
    set8(mmio, REG_AUTO_LLT_V1, AUTO_INIT_LLT_V1);
    if !poll_llt(mmio) {
        return false;
    }
    mmio.write8(REG_CR + 3, 0);
    true
}

// init_h2c: point the H2C queue head, read and tail registers at the reserved
// H2C pages, mark its info register, enable the drop-data path, then confirm the
// whole ring reads back free.
pub(super) fn init_h2c<M: Mmio>(mmio: &M) -> bool {
    rmw32(mmio, REG_H2C_HEAD, H2C_ADDR_KEEP, H2CQ_ADDR);
    rmw32(mmio, REG_H2C_READ_ADDR, H2C_ADDR_KEEP, H2CQ_ADDR);
    rmw32(mmio, REG_H2C_TAIL, H2C_ADDR_KEEP, H2CQ_TAIL);
    rmw8(mmio, REG_H2C_INFO, 0xFC, 0x01);
    rmw8(mmio, REG_H2C_INFO, 0xFB, 0x04);
    rmw8(mmio, REG_TXDMA_OFFSET_CHK + 1, 0x7F, DROP_DATA_EN_B1);

    let wp = mmio.read32(REG_H2C_PKT_WRITEADDR) & 0x3FFFF;
    let rp = mmio.read32(REG_H2C_PKT_READADDR) & 0x3FFFF;
    let free = if wp >= rp { H2CQ_SIZE - (wp - rp) } else { rp - wp };
    free == H2CQ_SIZE
}
