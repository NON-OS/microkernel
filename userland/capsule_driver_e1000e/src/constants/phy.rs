// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! MDIC fields and the PHY registers the bring-up touches. MDIC layout from
//! Linux e1000e defines.h (E1000_MDIC_*); PHY registers as (page, register)
//! from ich8lan.h `PHY_REG(page, reg)` and the IEEE 802.3 clause 22 set.

pub const MDIC_DATA_MASK: u32 = 0xFFFF;
pub const MDIC_REG_SHIFT: u32 = 16;
pub const MDIC_REG_MASK: u32 = 0x1F << MDIC_REG_SHIFT;
pub const MDIC_PHY_SHIFT: u32 = 21;
pub const MDIC_PHY_MASK: u32 = 0x1F << MDIC_PHY_SHIFT;
pub const MDIC_OP_WRITE: u32 = 1 << 26;
pub const MDIC_OP_READ: u32 = 2 << 26;
pub const MDIC_READY: u32 = 1 << 28;
pub const MDIC_ERROR: u32 = 1 << 30;

/// A PHY register: its page and its number within the page.
pub type PhyReg = (u16, u32);

pub const MII_BMCR: PhyReg = (0, 0);
pub const MII_PHYSID1: PhyReg = (0, 2);
pub const MII_PHYSID2: PhyReg = (0, 3);
pub const MII_ADVERTISE: PhyReg = (0, 4);
pub const MII_CTRL1000: PhyReg = (0, 9);
pub const BM_PORT_GEN_CFG: PhyReg = (769, 17);
pub const CV_SMB_CTRL: PhyReg = (769, 23);
pub const HV_PM_CTRL: PhyReg = (770, 17);
pub const I217_PHY_TIMEOUTS: PhyReg = (770, 21);
pub const I218_ULP_CONFIG1: PhyReg = (779, 16);

/// IGP01E1000_PHY_PAGE_SELECT; the page goes in as `page << IGP_PAGE_SHIFT`.
pub const PHY_PAGE_SELECT: u32 = 0x1F;
pub const IGP_PAGE_SHIFT: u32 = 5;
/// Registers above MAX_PHY_MULTI_PAGE_REG are the paged ones.
pub const MAX_PHY_MULTI_PAGE_REG: u32 = 0xF;
/// HV_INTC_FC_PAGE_START: pages from here on answer at PHY address 1, the
/// IEEE page 0 at address 2 (e1000_get_phy_addr_for_hv_page).
pub const HV_INTC_FC_PAGE_START: u16 = 768;
/// The 82574 and 82583 BM PHY answers at address 1 (e1000_init_phy_params_82571).
pub const BM_PHY_ADDR: u32 = 1;

pub const BMCR_ANRESTART: u16 = 1 << 9;
pub const BMCR_ANENABLE: u16 = 1 << 12;
/// 10 and 100 Mb/s, half and full: Linux AUTONEG_ADVERTISE_SPEED_DEFAULT
/// less 1000 half. Pause is not advertised: no flow control is set up.
pub const ADVERTISE_10_100: u16 = 0x01E0;
pub const ADVERTISE_PAUSE_BOTH: u16 = 0x0C00;
pub const ADVERTISE_1000FULL: u16 = 1 << 9;
pub const ADVERTISE_1000HALF: u16 = 1 << 8;

pub const CV_SMB_CTRL_FORCE_SMBUS: u16 = 1 << 0;
pub const HV_PM_CTRL_K1_ENABLE: u16 = 1 << 14;
pub const BM_WUC_HOST_WU_BIT: u16 = 1 << 4;
pub const I217_PHY_TIMEOUTS_K1_EXIT_TO_MASK: u16 = 0x0FC0;
pub const I217_PHY_TIMEOUTS_K1_EXIT_TO: u16 = 0x0F00;
pub const I218_ULP_CONFIG1_START: u16 = 1 << 0;
/// IND, STICKY_ULP, INBAND_EXIT, WOL_HOST, RESET_TO_SMBUS, EN_ULP_LANPHYPC,
/// DIS_CLR_STICKY_ON_PERST and DISABLE_SMB_PERST: what e1000_disable_ulp_lpt_lp
/// clears before it restarts the PHY's ULP configuration.
pub const I218_ULP_CONFIG1_CLEAR: u16 = 0x1D74;
