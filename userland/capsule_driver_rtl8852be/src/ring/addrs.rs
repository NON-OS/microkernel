// NONOS Operating System (AGPL-3.0-or-later)

//! Each DMA channel's registers: `rtw89_pci_ch_dma_addr_set` (Linux v6.12
//! rtw89 `pci.c`, the `DEF_TXCHADDRS` / `DEF_RXCHADDRS` rows, values from
//! `pci.h`), and the 8852B's BD RAM split, `rtw89_bd_ram_table_single`.
//! The 8852B leaves ACH4-7, CH10 and CH11 unused (`rtw8852be.c`,
//! `tx_dma_ch_mask`), so they have no row here.

/// One channel's ring registers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChAddrs {
    /// rtw89's channel number (`enum rtw89_tx_channel`), and the stop bit.
    pub ch: u8,
    pub num: u32,
    pub idx: u32,
    pub bdram: u32,
    pub desa_l: u32,
    /// BD RAM: start index, most and fewest descriptors fetched ahead.
    pub ram: (u8, u8, u8),
}

const fn tx(ch: u8, num: u32, idx: u32, bdram: u32, desa_l: u32, ram: (u8, u8, u8)) -> ChAddrs {
    ChAddrs { ch, num, idx, bdram, desa_l, ram }
}

pub const ACH0: ChAddrs = tx(0, 0x1024, 0x1058, 0x1200, 0x1110, (0, 5, 2));
pub const ACH1: ChAddrs = tx(1, 0x1026, 0x105c, 0x1204, 0x1118, (5, 5, 2));
pub const ACH2: ChAddrs = tx(2, 0x1028, 0x1060, 0x1208, 0x1120, (10, 5, 2));
pub const ACH3: ChAddrs = tx(3, 0x102a, 0x1064, 0x120c, 0x1128, (15, 5, 2));
/// Band 0's management queue (`RTW89_TXCH_CH8`).
pub const CH8: ChAddrs = tx(8, 0x1034, 0x1078, 0x1220, 0x1150, (20, 4, 1));
/// Band 0's high queue (`RTW89_TXCH_CH9`).
pub const CH9: ChAddrs = tx(9, 0x1036, 0x107c, 0x1224, 0x1158, (24, 4, 1));
/// The firmware command queue (`RTW89_TXCH_CH12`).
pub const CH12: ChAddrs = tx(12, 0x1038, 0x1080, 0x1228, 0x1160, (28, 4, 1));
/// Every transmit channel the 8852B uses, in rtw89's order.
pub const TX_CHANNELS: [ChAddrs; 7] = [ACH0, ACH1, ACH2, ACH3, CH8, CH9, CH12];

/// The receive queue and the release-report queue: (num, idx, desa_l).
pub const RXQ: (u32, u32, u32) = (0x1020, 0x1050, 0x1100);
pub const RPQ: (u32, u32, u32) = (0x1022, 0x1054, 0x1108);

/// A BD RAM control word: `BDRAM_SIDX_MASK`, `_MAX_MASK`, `_MIN_MASK`.
pub const fn bdram_word(ram: (u8, u8, u8)) -> u32 {
    ram.0 as u32 | (ram.1 as u32) << 8 | (ram.2 as u32) << 16
}

/// A ring index register's hardware read pointer, `TXBD_HW_IDX_MASK`.
pub const fn hw_idx(reg: u32) -> u32 {
    (reg >> 16) & 0xfff
}
