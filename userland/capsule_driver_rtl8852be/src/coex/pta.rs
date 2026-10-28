// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_mac_coex_init` (Linux v6.12 rtw89 `mac.c`) as the 8852B calls it
//! from `__rtw8852bx_btc_init_cfg` (`rtw8852b_common.c`): the packet
//! traffic arbiter in Realtek mode, BT signals routed inside the package.

use super::lte::{self, LteBusy, R_AX_LTE_SW_CFG_2};
use crate::regs::{Mmio, Rw};

pub const R_AX_GPIO_MUXCFG: u32 = 0x40;
const B_AX_ENBT: u8 = 0x20;
const B_AX_BTMODE_MASK: u8 = 0xc0;
const R_AX_BTC_FUNC_EN: u32 = 0xda20;
const B_AX_PTA_WL_TX_EN: u8 = 0x2;
const R_AX_BT_COEX_CFG_2: u32 = 0xda34;
const B_AX_GNT_BT_POLARITY_HI: u8 = 0x1; // 0x100 >> 8
pub const R_AX_CSR_MODE: u32 = 0xda40;
const B_AX_STATIS_BT_EN: u32 = 0x4;
const B_AX_WL_ACT_MSK: u32 = 0x8;
const B_AX_BT_CNT_RST: u32 = 0x1_0000;
const R_AX_TRXPTCL_RESP_0: u32 = 0xcc04;
const B_AX_RSP_CHK_BTCCA_HI: u8 = 0x2; // 0x2000000 >> 24
const R_AX_CCA_CFG_0: u32 = 0xc340;
const B_AX_BTCCA_EN: u16 = 0x20;
const B_AX_BTCCA_BRK_TXOP_EN: u16 = 0x200;
const B_AX_WL_RX_CTRL: u32 = 0x100;
const R_AX_TDMA_MODE: u32 = 0xda4c;
const B_AX_RTK_BT_ENABLE: u8 = 0x1;
const R_AX_BT_COEX_CFG_5: u32 = 0xda6c;
const B_AX_BT_RPT_SAMPLE_RATE_MASK: u8 = 0x3f;
const MAC_AX_RTK_RATE: u8 = 5;

pub fn coex_init(r: &impl Mmio) -> Result<(), LteBusy> {
    r.set8(R_AX_GPIO_MUXCFG, B_AX_ENBT);
    r.set8(R_AX_BTC_FUNC_EN, B_AX_PTA_WL_TX_EN);
    r.set8(R_AX_BT_COEX_CFG_2 + 1, B_AX_GNT_BT_POLARITY_HI);
    r.set8(R_AX_CSR_MODE, (B_AX_STATIS_BT_EN | B_AX_WL_ACT_MSK) as u8);
    r.set8(R_AX_CSR_MODE + 2, (B_AX_BT_CNT_RST >> 16) as u8);
    r.clr8(R_AX_TRXPTCL_RESP_0 + 3, B_AX_RSP_CHK_BTCCA_HI);
    let cca = r.read16(R_AX_CCA_CFG_0);
    r.write16(R_AX_CCA_CFG_0, (cca | B_AX_BTCCA_EN) & !B_AX_BTCCA_BRK_TXOP_EN);

    let v = lte::read(r, R_AX_LTE_SW_CFG_2)?;
    lte::write(r, R_AX_LTE_SW_CFG_2, v & B_AX_WL_RX_CTRL)?;

    // RTK mode: BT mode 0/3 (field value 0), RTK BT on, report rate 5.
    r.mask8(R_AX_GPIO_MUXCFG, B_AX_BTMODE_MASK, 0);
    r.set8(R_AX_TDMA_MODE, B_AX_RTK_BT_ENABLE);
    r.mask8(R_AX_BT_COEX_CFG_5, B_AX_BT_RPT_SAMPLE_RATE_MASK, MAC_AX_RTK_RATE);
    // Inner direction: bit 1 set, bit 2 clear.
    let v = r.read8(R_AX_GPIO_MUXCFG + 1);
    r.write8(R_AX_GPIO_MUXCFG + 1, (v & !0x4) | 0x2);
    Ok(())
}

/// The BT counter enable `__rtw8852bx_btc_init_cfg` ends with.
pub fn bt_counter_on(r: &impl Mmio) {
    r.set32(R_AX_CSR_MODE, B_AX_BT_CNT_RST | B_AX_STATIS_BT_EN);
}
