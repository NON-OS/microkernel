// NONOS Operating System (AGPL-3.0-or-later)

//! The power sequence's registers and bits, Linux v6.12 rtw89 `reg.h` and
//! `mac.h` (BSD-3-Clause option; notice in `docs/licence.md`).

pub const R_AX_SYS_ISO_CTRL: u32 = 0x0;
pub const B_AX_ISO_EB2CORE: u32 = 0x100;
pub const B_AX_PWC_EV2EF_B15: u32 = 0x8000;
pub const B_AX_PWC_EV2EF_B14: u32 = 0x4000;
pub const R_AX_SYS_FUNC_EN: u32 = 0x2;
pub const B_AX_FEN_BB_GLB_RSTN: u8 = 0x2;
pub const B_AX_FEN_BBRSTB: u8 = 0x1;
pub const R_AX_SYS_PW_CTRL: u32 = 0x4;
pub const B_AX_AFSM_WLSUS_EN: u32 = 0x800;
pub const B_AX_AFSM_PCIE_SUS_EN: u32 = 0x1000;
pub const B_AX_DIS_WLBT_PDNSUSEN_SOPC: u32 = 0x40000;
pub const B_AX_APDM_HPDN: u32 = 0x8000;
pub const B_AX_APFM_SWLPS: u32 = 0x400;
pub const B_AX_APFM_OFFMAC: u32 = 0x200;
pub const B_AX_APFN_ONMAC: u32 = 0x100;
pub const B_AX_RDY_SYSPWR: u32 = 0x20000;
pub const B_AX_EN_WLON: u32 = 0x10000;
pub const R_AX_SYS_SWR_CTRL1: u32 = 0x10;
pub const B_AX_SYM_CTRL_SPS_PWMFREQ: u32 = 0x400;
pub const R_AX_SYS_ADIE_PAD_PWR_CTRL: u32 = 0x18;
pub const B_AX_SYM_PADPDN_WL_PTA_1P3: u32 = 0x40;
pub const B_AX_SYM_PADPDN_WL_RFC_1P3: u32 = 0x20;
pub const R_AX_AFE_LDO_CTRL: u32 = 0x20;
pub const B_AX_AON_OFF_PC_EN: u32 = 0x80_0000;
pub const R_AX_SYS_SDIO_CTRL: u32 = 0x70;
pub const B_AX_PCIE_CALIB_EN_V1: u32 = 0x1000;
pub const R_AX_HCI_LDO_CTRL: u32 = 0x7a;
pub const B_AX_R_AX_VADJ_MASK: u16 = 0xf;
pub const R_AX_PLATFORM_ENABLE: u32 = 0x88;
pub const B_AX_PLATFORM_EN: u8 = 0x1;
pub const R_AX_WLLPS_CTRL: u32 = 0x90;
pub const B_AX_DIS_WLBT_LPSEN_LOPC: u32 = 0x2;
pub const SW_LPS_OPTION: u32 = 0x1a0b2;
pub const R_AX_SCOREBOARD: u32 = 0xac;
pub const MAC_AX_NOTIFY_TP_MAJOR: u8 = 0x81;
pub const MAC_AX_NOTIFY_PWR_MAJOR: u8 = 0x80;
pub const R_AX_PMC_DBG_CTRL2: u32 = 0xcc;
pub const B_AX_SYSON_DIS_PMCR_AX_WRMSK: u32 = 0x4;
pub const R_AX_SYS_CFG1: u32 = 0xf0;
pub const B_AX_CHIP_VER_MASK: u32 = 0xf000;
pub const R_AX_SPS_DIG_ON_CTRL0: u32 = 0x200;
pub const B_AX_VOL_L1_MASK: u32 = 0xf;
pub const B_AX_VREFPFM_L_MASK: u32 = 0x3c0_0000;
pub const B_AX_REG_ZCDC_H_MASK: u32 = 0x6_0000;
pub const R_AX_SPS_ANA_ON_CTRL2: u32 = 0x228;
pub const RTL8852B_RFE_05_SPS_ANA: u16 = 0x4a82;
pub const R_AX_EECS_EESK_FUNC_SEL: u32 = 0x2d8;
pub const B_AX_PINMUX_EESK_FUNC_SEL_MASK: u32 = 0xf0;
pub const PINMUX_EESK_FUNC_SEL_BT_LOG: u32 = 0x1;
pub const R_AX_WLRF_CTRL: u32 = 0x2f0;
pub const B_AX_AFC_AFEDIG: u32 = 0x2_0000;
pub const R_AX_IC_PWR_STATE: u32 = 0x3f0;
pub const B_AX_WLMAC_PWR_STE_MASK: u32 = 0x300;
pub const R_AX_SPS_DIG_OFF_CTRL0: u32 = 0x400;
pub const B_AX_C1_L1_MASK: u32 = 0x3;
pub const B_AX_C3_L1_MASK: u32 = 0x30;
/// `rtw89_mac_power_switch`'s "already powered" state of the MAC.
pub const PWR_ACT: u32 = 1;
/// `chip->rf_path_num` aside, the cut ids `hal.cv` takes (`core.h`).
pub const CHIP_CBV: u8 = 1;
