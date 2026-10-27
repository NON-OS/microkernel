/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Chip addresses the driver touches, as facts from Linux v6.12
//! `mt76/mt792x_regs.h` and `mt76/mt7921/regs.h`.

/// The host's WFDMA0 block (`MT_WFDMA0_BASE`).
pub const WFDMA0: u32 = 0xd4000;
pub const WFDMA0_RST: u32 = WFDMA0 + 0x100;
pub const HOST_INT_ENA: u32 = WFDMA0 + 0x204;
pub const GLO_CFG: u32 = WFDMA0 + 0x208;
pub const RST_DTX_PTR: u32 = WFDMA0 + 0x20c;
pub const GLO_CFG_EXT0: u32 = WFDMA0 + 0x2b0;
pub const PRI_DLY_INT_CFG0: u32 = WFDMA0 + 0x2f0;
/// Each ring's four registers, sixteen bytes apart (`mt76_queue_regs`).
pub const TX_RING_BASE: u32 = WFDMA0 + 0x300;
pub const RX_RING_BASE: u32 = WFDMA0 + 0x500;
pub const TX_RING_EXT_CTRL: u32 = WFDMA0 + 0x600;
pub const RX_RING_EXT_CTRL: u32 = WFDMA0 + 0x680;

/// `MT_CONN_ON_LPCTL` and its bits: host ownership of the chip.
pub const CONN_ON_LPCTL: u32 = 0x7c060010;
pub const LPCR_HOST_SET_OWN: u32 = 1 << 0;
pub const LPCR_HOST_CLR_OWN: u32 = 1 << 1;
pub const LPCR_HOST_OWN_SYNC: u32 = 1 << 2;
/// `MT_CONN_ON_MISC`: the firmware's ready bits, `MT_TOP_MISC2_FW_N9_RDY`.
pub const CONN_ON_MISC: u32 = 0x7c0600f0;
pub const FW_N9_RDY: u32 = 0x3;
/// `MT_TOP_LPCR_HOST_BAND0` (`mt7921e_driver_own`).
pub const TOP_LPCR_HOST_BAND0: u32 = 0x18060010;
pub const TOP_LPCR_HOST_FW_OWN: u32 = 1 << 0;
pub const TOP_LPCR_HOST_DRV_OWN: u32 = 1 << 1;
/// The chip's identity (`mt7921_pci_probe`).
pub const HW_BOUND: u32 = 0x70010020;
pub const HW_CHIPID: u32 = 0x70010200;
pub const HW_REV: u32 = 0x70010204;
/// The Wi-Fi subsystem reset of the MT7921 family (`mt792x_wfsys_reset`).
pub const WFSYS_SW_RST: u32 = 0x18000140;
pub const WFSYS_SW_RST_B: u32 = 1 << 0;
pub const WFSYS_SW_INIT_DONE: u32 = 1 << 4;
/// The PCIe MAC: interrupt enable and the L0s switch.
pub const PCIE_MAC_INT_ENABLE: u32 = 0x10188;
pub const PCIE_MAC_PM: u32 = 0x10194;
pub const PCIE_MAC_PM_L0S_DIS: u32 = 1 << 8;
/// The DMA scheduler's bypass (`mt792x_dma_disable`).
pub const DMASHDL_SW_CONTROL: u32 = 0x7c026004;
pub const DMASHDL_BYPASS: u32 = 1 << 28;
/// The WFDMA reinit flag the firmware reads (`MT_WFDMA_DUMMY_CR`).
pub const WFDMA_DUMMY_CR: u32 = 0x54000120;
pub const WFDMA_NEED_REINIT: u32 = 1 << 1;
/// The firmware's operation mode, normal before download (`MT_SWDEF_MODE`).
pub const SWDEF_MODE: u32 = 0x41f23c;
/// The MAC data path's receive control (`MT_MDP_DCR0`, `MT_MDP_DCR1`).
pub const MDP_DCR0: u32 = 0x820cd000;
pub const MDP_DCR0_DAMSDU_EN: u32 = 1 << 15;
pub const MDP_DCR0_RX_HDR_TRANS_EN: u32 = 1 << 19;
pub const MDP_DCR1: u32 = 0x820cd004;
pub const MDP_DCR1_MAX_RX_LEN: u32 = 0xfff8;
/// The L1 remap register (`MT_HIF_REMAP_L1`) and the window it opens.
pub const HIF_REMAP_L1: u32 = 0xfe24c;
pub const HIF_REMAP_BASE_L1: u32 = 0x40000;
