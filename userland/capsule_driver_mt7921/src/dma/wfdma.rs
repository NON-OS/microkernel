/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The WFDMA engine on and off: Linux v6.12 `mt792x_dma.c`,
//! `mt792x_dma_disable`, `mt792x_dma_prefetch` (the MT7921 family's
//! branch) and `mt792x_dma_enable`. The host interrupt enables stay zero:
//! this driver polls, so it binds no interrupt and asks for none.

use super::prefetch::PREFETCH;
use crate::hw::addr::*;
use crate::hw::{poll, Regs};

const TX_DMA_EN: u32 = 1 << 0;
const TX_DMA_BUSY: u32 = 1 << 1;
const RX_DMA_EN: u32 = 1 << 2;
const RX_DMA_BUSY: u32 = 1 << 3;
const DMA_SIZE_3: u32 = 3 << 4;
const TX_WB_DDONE: u32 = 1 << 6;
const FIFO_DIS_CHECK: u32 = 1 << 11;
const FIFO_LITTLE_ENDIAN: u32 = 1 << 12;
const RX_WB_DDONE: u32 = 1 << 13;
const CSR_DISP_BASE_PTR_CHAIN_EN: u32 = 1 << 15;
const OMIT_RX_INFO_PFET2: u32 = 1 << 21;
const OMIT_RX_INFO: u32 = 1 << 27;
const OMIT_TX_INFO: u32 = 1 << 28;
const CLK_GAT_DIS: u32 = 1 << 30;
const TX_DMASHDL_ENABLE: u32 = 1 << 6;
const RST_LOGIC: u32 = 1 << 4;
const RST_DMASHDL_ALL: u32 = 1 << 5;

/// Stop both directions and wait for the engine to go idle; with `force`,
/// pulse the logic and scheduler resets. Whether it went idle in 100 ms.
pub fn disable(regs: &Regs, force: bool) -> bool {
    let off = TX_DMA_EN | RX_DMA_EN | CSR_DISP_BASE_PTR_CHAIN_EN;
    regs.clear(GLO_CFG, off | OMIT_TX_INFO | OMIT_RX_INFO | OMIT_RX_INFO_PFET2);
    if !poll(regs, GLO_CFG, TX_DMA_BUSY | RX_DMA_BUSY, 0, 100) {
        return false;
    }
    regs.clear(GLO_CFG_EXT0, TX_DMASHDL_ENABLE);
    regs.set(DMASHDL_SW_CONTROL, DMASHDL_BYPASS);
    if force {
        regs.clear(WFDMA0_RST, RST_DMASHDL_ALL | RST_LOGIC);
        regs.set(WFDMA0_RST, RST_DMASHDL_ALL | RST_LOGIC);
    }
    true
}

/// The engine on, its rings already reset and armed.
pub fn enable(regs: &Regs) {
    for (reg, val) in PREFETCH {
        regs.wr(reg, val);
    }
    regs.wr(RST_DTX_PTR, u32::MAX);
    regs.wr(PRI_DLY_INT_CFG0, 0);
    let wb = TX_WB_DDONE | RX_WB_DDONE | FIFO_LITTLE_ENDIAN | FIFO_DIS_CHECK | CLK_GAT_DIS;
    let omit = OMIT_TX_INFO | OMIT_RX_INFO_PFET2;
    regs.set(GLO_CFG, wb | omit | DMA_SIZE_3 | CSR_DISP_BASE_PTR_CHAIN_EN);
    regs.set(GLO_CFG, TX_DMA_EN | RX_DMA_EN);
    regs.set(WFDMA_DUMMY_CR, WFDMA_NEED_REINIT);
}
