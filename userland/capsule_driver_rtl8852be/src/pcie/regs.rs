// NONOS Operating System (AGPL-3.0-or-later)

//! The PCIe glue's registers, Linux v6.12 rtw89 `pci.h` and `reg.h`
//! (BSD-3-Clause option; notice in `docs/licence.md`). The LTR, MDIO and
//! config-space ones are in `regs_ltr`.

pub use super::regs_ltr::*;

pub const R_AX_SYS_PW_CTRL: u32 = 0x4;
pub const B_AX_PSUS_OFF_CAPC_EN: u32 = 0x4000;
pub const R_AX_SYS_SDIO_CTRL: u32 = 0x70;
pub const B_AX_PCIE_DIS_L2_CTRL_LDO_HCI: u32 = 0x8000;
pub const B_AX_PCIE_DIS_WLSUS_AFT_PDN: u32 = 0x4000;
pub const R_AX_HCI_OPT_CTRL: u32 = 0x74;
pub const BIT_WAKE_CTRL: u32 = 0x20;
pub const R_AX_PCIE_INIT_CFG1: u32 = 0x1000;
pub const B_AX_RST_BDRAM: u32 = 0x8;
pub const B_AX_TXHCI_EN: u32 = 0x800;
pub const B_AX_LATENCY_CONTROL: u32 = 0x1000;
pub const B_AX_RXHCI_EN: u32 = 0x2000;
pub const B_AX_RXBD_MODE: u32 = 0x4_0000;
pub const B_AX_PCIE_MAX_TXDMA_MASK: u32 = 0x700;
pub const B_AX_PCIE_MAX_RXDMA_MASK: u32 = 0x1_c000;
pub const B_AX_PCIE_TXRST_KEEP_REG: u32 = 0x40_0000;
pub const B_AX_PCIE_RXRST_KEEP_REG: u32 = 0x80_0000;
pub const R_AX_PCIE_INIT_CFG2: u32 = 0x1004;
pub const B_AX_WD_ITVL_IDLE: u32 = 0xf00_0000;
pub const B_AX_WD_ITVL_ACT: u32 = 0xf_0000;
pub const R_AX_PCIE_PS_CTRL: u32 = 0x1008;
pub const B_AX_L1OFF_PWR_OFF_EN: u32 = 0x20;
pub const R_AX_PCIE_DMA_STOP1: u32 = 0x1010;
pub const B_AX_STOP_WPDMA: u32 = 0x8_0000;
pub const B_AX_STOP_PCIEIO: u32 = 0x10_0000;
pub const B_AX_STOP_CH12: u32 = 0x4_0000;
/// ACH0-3, CH8, CH9 and CH12 (`B_AX_TX_STOP1_MASK_V1`).
pub const B_AX_TX_STOP1_MASK_V1: u32 = 0x7_0f00;
pub const R_AX_TXBD_RWPTR_CLR1: u32 = 0x1014;
/// ACH0-3, CH8, CH9, CH12's pointer clears.
pub const TX_CLR_8852B: u32 = 0x70f;
pub const R_AX_RXBD_RWPTR_CLR: u32 = 0x1018;
pub const RX_CLR: u32 = 0x3;
pub const R_AX_PCIE_DMA_BUSY1: u32 = 0x101c;
/// ACH0-3, CH8, CH9, CH12 busy (`DMA_BUSY1_CHECK_V1`).
pub const DMA_BUSY1_CHECK_V1: u32 = 0x7_0f00;
/// RXQ and RPQ busy.
pub const RX_BUSY: u32 = 0x3;
pub const R_AX_PCIE_DBG_CTRL: u32 = 0x11c0;
pub const B_AX_ASFF_FULL_NO_STK: u32 = 0x2;
pub const B_AX_EN_STUCK_DBG: u32 = 0x1;
pub const R_AX_LBC_WATCHDOG: u32 = 0x11d8;
pub const B_AX_LBC_TIMER: u32 = 0xf0;
pub const B_AX_LBC_FLAG: u32 = 0x2;
pub const B_AX_LBC_EN: u32 = 0x1;
pub const MAC_AX_LBC_TMR_2MS: u32 = 0x8;
pub const R_AX_PCIE_EXP_CTRL: u32 = 0x13f0;
pub const B_AX_SIC_EN_FORCE_CLKREQ: u32 = 0x10;
pub const B_AX_MAX_TAG_NUM: u32 = 0x7_0000;
pub const R_AX_TX_ADDRESS_INFO_MODE_SETTING: u32 = 0x8810;
pub const B_AX_HOST_ADDR_INFO_8B_SEL: u32 = 0x1;
pub const R_AX_PKTIN_SETTING: u32 = 0x9a00;
pub const B_AX_WD_ADDR_INFO_LENGTH: u32 = 0x2;
