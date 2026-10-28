// NONOS Operating System (AGPL-3.0-or-later)

//! The PCIe glue's LTR registers, the PHY's MDIO registers, and the config
//! bytes read through DBI: Linux v6.12 rtw89 `pci.h` and `reg.h`
//! (BSD-3-Clause option; notice in `docs/licence.md`).

pub const R_AX_LTR_CTRL_0: u32 = 0x8410;
pub const R_AX_LTR_CTRL_1: u32 = 0x8414;
pub const R_AX_LTR_IDLE_LATENCY: u32 = 0x8418;
pub const R_AX_LTR_ACTIVE_LATENCY: u32 = 0x841c;
pub const B_AX_LTR_EN_BITS: u32 = 0x43;
pub const B_AX_LTR_SPACE_IDX_MASK: u32 = 0x3000;
pub const B_AX_LTR_IDLE_TIMER_IDX_MASK: u32 = 0x700;
pub const B_AX_LTR_RX0_TH_MASK: u32 = 0xfff;
pub const B_AX_LTR_RX1_TH_MASK: u32 = 0xfff_0000;
/// The PHY's MDIO registers and fields the 8852B touches.
pub const RAC_REG_REV2: u8 = 0x1b;
pub const BAC_CMU_EN_DLY_MASK: u16 = 0xf000;
pub const RAC_REG_FLD_0: u8 = 0x1d;
pub const BAC_AUTOK_N_MASK: u16 = 0xc;
pub const RAC_CTRL_PPR_V1: u8 = 0x30;
pub const B_AX_CALIB_EN: u16 = 0x2000;
/// Config-space bytes read through DBI: the link rate, the L1 control.
pub const RTW89_PCIE_PHY_RATE: u16 = 0x82;
pub const RTW89_PCIE_L1_CTRL: u16 = 0x719;
pub const RTW89_PCIE_BIT_L1: u8 = 0x8;
