// NONOS Operating System (AGPL-3.0-or-later)

//! The page boundaries and the H2C queue's place, derived from the page counts.

use super::pages::{
    C2H_PKT_BUF, CPU_INSTRUCTION, CSI_BUF, EXQ_NUM, FW_TXBUF, H2C_EXTRAINFO, H2C_STATICINFO,
    HQ_NUM, LQ_NUM, NQ_NUM, RSVD_DRV_PG_NUM, RSVD_H2CQ_NUM, RXFF_SIZE, TXFF_SIZE,
    TX_PAGE_SIZE_SHIFT,
};

const GAPQ_NUM: u16 = 1;

// Total transmit FIFO pages, and how many are left for the addressable queues
// once the reserved pages are taken out.
const TXFF_PG_NUM: u32 = TXFF_SIZE >> TX_PAGE_SIZE_SHIFT;

const RSVD_PG_NUM: u32 = RSVD_DRV_PG_NUM
    + H2C_EXTRAINFO
    + H2C_STATICINFO
    + RSVD_H2CQ_NUM
    + CPU_INSTRUCTION
    + FW_TXBUF
    + CSI_BUF;

const ACQ_PG_NUM: u32 = TXFF_PG_NUM - RSVD_PG_NUM;

// The reserved-page boundary the beacon and public queues are bounded against.
pub(super) const RSVD_BOUNDARY: u16 = (TXFF_PG_NUM - RSVD_PG_NUM) as u16;

// The public queue takes whatever pages the per-priority queues do not.
pub(super) const PUBQ_NUM: u16 = ACQ_PG_NUM as u16 - HQ_NUM - LQ_NUM - NQ_NUM - EXQ_NUM - GAPQ_NUM;

// The receive FIFO boundary leaves room for the C2H report buffer.
pub(super) const RXFF_BNDY: u32 = RXFF_SIZE - C2H_PKT_BUF - 1;

// The H2C queue lives just below the firmware TX buffer, walking down from the
// top of the FIFO exactly as rtw88 sites it.
const CSIBUF_ADDR: u32 = TXFF_PG_NUM - CSI_BUF;
const FW_TXBUF_ADDR: u32 = CSIBUF_ADDR - FW_TXBUF;
const CPU_INSTR_ADDR: u32 = FW_TXBUF_ADDR - CPU_INSTRUCTION;
const H2CQ_PAGE: u32 = CPU_INSTR_ADDR - RSVD_H2CQ_NUM;
pub(super) const H2CQ_ADDR: u32 = H2CQ_PAGE << TX_PAGE_SIZE_SHIFT;
pub(super) const H2CQ_SIZE: u32 = RSVD_H2CQ_NUM << TX_PAGE_SIZE_SHIFT;
pub(super) const H2CQ_TAIL: u32 = H2CQ_ADDR + H2CQ_SIZE;

// The H2C address fields keep their top bits; only the low 18 hold the address.
pub(super) const H2C_ADDR_KEEP: u32 = 0xFFFC_0000;

// rqpn_table_8821c[1] (PCIe): the priority-to-DMA-queue mapping, packed into
// REG_TXDMA_PQ_MAP two bits per queue. vo/vi=NORMAL(2), be/bk=LOW(1),
// mg=EXTRA(0), hi=HIGH(3).
pub(super) const fn txdma_pq_map() -> u16 {
    const VO: u16 = 2;
    const VI: u16 = 2;
    const BE: u16 = 1;
    const BK: u16 = 1;
    const MG: u16 = 0;
    const HI: u16 = 3;
    (VO << 4) | (VI << 6) | (BE << 8) | (BK << 10) | (MG << 12) | (HI << 14)
}
