// NONOS Operating System (AGPL-3.0-or-later)

//! The FIFO geometry and the queues' page counts, rtw88's for the 8821C on PCIe.

// Beacon-queue full marker for the 11ac H2C path.
pub(super) const H2CQ_FULL: u32 = 1 << 31;

// BIT_EN_WR_FREE_TAIL (BIT20) seen in the third byte of REG_FWHW_TXQ_CTRL.
pub(super) const EN_WR_FREE_TAIL_B2: u8 = 0x10;

// Trigger and status of the automatic link-list-table init.
pub(super) const AUTO_INIT_LLT_V1: u8 = 0x01;

// BIT_DROP_DATA_EN (BIT15) in the high byte of REG_TXDMA_OFFSET_CHK.
pub(super) const DROP_DATA_EN_B1: u8 = 0x80;

// FIFO geometry for the 8821c (rtw8821c.c chip params, main.h page size).
pub(super) const TX_PAGE_SIZE_SHIFT: u32 = 7;
pub(super) const TXFF_SIZE: u32 = 65536;
pub(super) const RXFF_SIZE: u32 = 16384;
pub(super) const C2H_PKT_BUF: u32 = 256;

// Reserved driver/firmware pages that sit above the addressable queues.
pub(super) const RSVD_DRV_PG_NUM: u32 = 8;
pub(super) const H2C_EXTRAINFO: u32 = 24;
pub(super) const H2C_STATICINFO: u32 = 8;
pub(super) const RSVD_H2CQ_NUM: u32 = 8;
pub(super) const CPU_INSTRUCTION: u32 = 0;
pub(super) const FW_TXBUF: u32 = 4;
pub(super) const CSI_BUF: u32 = 0;

// page_table_8821c[1] (PCIe): the per-priority queue page counts.
pub(super) const HQ_NUM: u16 = 16;
pub(super) const NQ_NUM: u16 = 16;
pub(super) const LQ_NUM: u16 = 16;
pub(super) const EXQ_NUM: u16 = 14;
