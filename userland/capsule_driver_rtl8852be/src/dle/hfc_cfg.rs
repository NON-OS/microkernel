// NONOS Operating System (AGPL-3.0-or-later)

//! The 8852B's host flow control parameters on PCIe (Linux v6.12 rtw89
//! `rtw8852b.c`: `rtw8852b_hfc_chcfg_pcie`, `rtw8852b_hfc_pubcfg_pcie`;
//! `mac.c`: `rtw89_mac_size.hfc_preccfg_pcie`), and the registers they go
//! to (`rtw8852b_page_regs`, `reg.h`).

/// One DMA channel's page budget: min, max, group 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChCfg {
    pub min: u16,
    pub max: u16,
    pub grp1: bool,
}

const fn ch(min: u16, max: u16) -> ChCfg {
    ChCfg { min, max, grp1: false }
}

/// ACH0..ACH7, B0MGQ, B0HIQ, B1MGQ, B1HIQ, in rtw89's order.
pub const CH_CFG: [ChCfg; 12] = [
    ch(5, 341),
    ch(5, 341),
    ch(4, 342),
    ch(4, 342),
    ch(0, 0),
    ch(0, 0),
    ch(0, 0),
    ch(0, 0),
    ch(4, 342),
    ch(4, 342),
    ch(0, 0),
    ch(0, 0),
];
/// Channels the 8852B leaves unused: ACH4-7, B1MGQ, B1HIQ (`dma_ch_mask`).
pub const DMA_CH_MASK: u16 = 0x0cf0;
/// Group 0, group 1, public maximum, WP threshold.
pub const PUB: (u16, u16, u16, u16) = (446, 0, 446, 0);
/// ch011_prec, h2c_prec, wp_ch07_prec, wp_ch811_prec, ch011_full_cond,
/// h2c_full_cond, wp_ch07_full_cond, wp_ch811_full_cond.
pub const PREC: (u16, u16, u16, u16, u8, u8, u8, u8) = (2, 40, 0, 0, 1, 0, 0, 0);
/// `RTW89_HCIFC_POH`, the PCIe mode.
pub const MODE_POH: u32 = 0;

pub const R_AX_HCI_FC_CTRL: u32 = 0x8a00;
pub const R_AX_CH_PAGE_CTRL: u32 = 0x8a04;
pub const R_AX_ACH0_PAGE_CTRL: u32 = 0x8a10;
pub const R_AX_ACH0_PAGE_INFO: u32 = 0x8a50;
pub const R_AX_PUB_PAGE_INFO3: u32 = 0x8a8c;
pub const R_AX_PUB_PAGE_CTRL1: u32 = 0x8a90;
pub const R_AX_PUB_PAGE_CTRL2: u32 = 0x8a94;
pub const R_AX_PUB_PAGE_INFO1: u32 = 0x8a98;
pub const R_AX_PUB_PAGE_INFO2: u32 = 0x8a9c;
pub const R_AX_WP_PAGE_CTRL1: u32 = 0x8aa0;
pub const R_AX_WP_PAGE_CTRL2: u32 = 0x8aa4;
pub const HCI_FC_EN: u32 = 0x1;
pub const HCI_FC_MODE: u32 = 0x6;
pub const HCI_FC_CH12_EN: u32 = 0x8;
pub const WD_FULL_COND: u32 = 0x30;
pub const WP_CH07_FULL_COND: u32 = 0xc0;
pub const WP_CH811_FULL_COND: u32 = 0x300;
pub const CH12_FULL_COND: u32 = 0xc00;
pub const PREC_CH011: u32 = 0x1ff;
pub const PREC_CH12: u32 = 0x1ff_0000;
pub const PG_MIN: u32 = 0x1fff;
pub const PG_MAX: u32 = 0x1fff_0000;
pub const PG_GRP: u32 = 0x8000_0000;
pub const G0: u32 = 0x1fff;
pub const G1: u32 = 0x1fff_0000;
pub const PUB_AVAL: u32 = 0x1fff;
