// NONOS Operating System (AGPL-3.0-or-later)

//! What rtw88's PCIe layer does for the 8822C alone (pci.c, Linux v6.12).
//! Its gen1 and gen2 PHY parameter tables are empty for this chip
//! (`pcie_gen1_param_8822c`), so no MDIO write is made.

use super::dbi;
use crate::regs::Mmio;

/// `RTK_PCIE_CLKDLY_CTRL`: the chip's reference-clock delay.
const RTK_PCIE_CLKDLY_CTRL: u16 = 0x0725;
/// `REG_HCI_MIX_CFG` and `BIT_PCIE_EMAC_PDN_AUX_TO_FAST_CLK`.
const REG_HCI_MIX_CFG: usize = 0x03FC;
const EMAC_PDN_AUX_TO_FAST_CLK: u32 = 1 << 26;
/// `RTW_CHIP_VER_CUT_D`: the first cut that takes the bit above.
const CUT_D: u8 = 3;
/// `REG_ANAPARSW_MAC_0` and `BIT_CF_L_V2` (bits 29:28).
const REG_ANAPARSW_MAC_0: usize = 0x1010;
const CF_L_V2_MASK: u32 = 0x3000_0000;

/// rtw_pci_link_cfg: "RTL8822CE has enabled REFCLK auto calibration, it does
/// not need to add clock delay". False when the DBI write never completed.
pub fn clock_delay_off<M: Mmio>(mmio: &M) -> bool {
    dbi::write8(mmio, RTK_PCIE_CLKDLY_CTRL, 0)
}

/// rtw_pci_phy_cfg's tail, once the efuse is read: front end 5 sets
/// `BIT_CF_L_V2` to 1.
pub fn board<M: Mmio>(mmio: &M, rfe: u8) {
    if rfe == 5 {
        let v = mmio.read32(REG_ANAPARSW_MAC_0) & !CF_L_V2_MASK;
        mmio.write32(REG_ANAPARSW_MAC_0, v | (1 << 28));
    }
}

/// rtw_pci_interface_cfg, after the MAC init: cut D on sets
/// `BIT_PCIE_EMAC_PDN_AUX_TO_FAST_CLK`.
pub fn after_mac<M: Mmio>(mmio: &M, cut: u8) {
    if cut >= CUT_D {
        let v = mmio.read32(REG_HCI_MIX_CFG);
        mmio.write32(REG_HCI_MIX_CFG, v | EMAC_PDN_AUX_TO_FAST_CLK);
    }
}
