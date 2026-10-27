// NONOS Operating System (AGPL-3.0-or-later)

//! The FIFO geometry and the queues' page counts, rtw88's for the 8822C on
//! PCIe: `rtw8822c_hw_spec` (txff 256 KiB, rxff 24 KiB, 16 driver pages, 50
//! CSI pages) and `page_table_8822c[1]` (64 pages each to the high, normal,
//! low and extra queues). The reserved-page counts are mac.h's, the same on
//! every 11ac chip. The bits are reg.h's.

pub(super) const H2CQ_FULL: u32 = 1 << 31;
pub(super) const EN_WR_FREE_TAIL_B2: u8 = 0x10;
pub(super) const AUTO_INIT_LLT_V1: u8 = 0x01;
pub(super) const DROP_DATA_EN_B1: u8 = 0x80;

pub(super) const TX_PAGE_SIZE_SHIFT: u32 = 7;
pub const TXFF_SIZE: u32 = 262144;
pub const RXFF_SIZE: u32 = 24576;
pub(super) const C2H_PKT_BUF: u32 = 256;

pub(super) const RSVD_DRV_PG_NUM: u32 = 16;
pub(super) const H2C_EXTRAINFO: u32 = 24;
pub(super) const H2C_STATICINFO: u32 = 8;
pub(super) const RSVD_H2CQ_NUM: u32 = 8;
pub(super) const CPU_INSTRUCTION: u32 = 0;
pub(super) const FW_TXBUF: u32 = 4;
pub(super) const CSI_BUF: u32 = 50;

pub(super) const HQ_NUM: u16 = 64;
pub(super) const NQ_NUM: u16 = 64;
pub(super) const LQ_NUM: u16 = 64;
pub(super) const EXQ_NUM: u16 = 64;
