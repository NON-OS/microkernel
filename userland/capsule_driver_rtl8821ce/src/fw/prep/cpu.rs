// NONOS Operating System (AGPL-3.0-or-later)

//! The 8051 halted and started, and the platform reset.

use super::super::regs::{
    CPU_CLK_EN_HI, FEN_CPUEN_HI, MCUFWDL_EN, REG_CPU_DMEM_CON, REG_MCUFW_CTRL, REG_RSV_CTRL,
    REG_SYS_CLK_CTRL, REG_SYS_FUNC_EN, WLMCU_IOIF_HI, WL_PLATFORM_RST_B2,
};
use crate::regs::Mmio;

// Stop the 8051: drop its run enable, then its IO interface.
pub(super) fn halt_cpu<M: Mmio>(mmio: &M) {
    clr8(mmio, REG_SYS_FUNC_EN + 1, FEN_CPUEN_HI);
    clr8(mmio, REG_RSV_CTRL + 1, WLMCU_IOIF_HI);
}

// Release the 8051: restore its IO interface, then its run enable.
pub(super) fn start_cpu<M: Mmio>(mmio: &M) {
    set8(mmio, REG_RSV_CTRL + 1, WLMCU_IOIF_HI);
    set8(mmio, REG_SYS_FUNC_EN + 1, FEN_CPUEN_HI);
}

// Leave the 8051 halted after a download that did not finish: download mode
// off, the CPU stopped, its clock off and the WL platform held in reset.
pub(super) fn halt<M: Mmio>(mmio: &M) {
    let ctrl = mmio.read16(REG_MCUFW_CTRL);
    mmio.write16(REG_MCUFW_CTRL, ctrl & !MCUFWDL_EN);
    halt_cpu(mmio);
    clr8(mmio, REG_SYS_CLK_CTRL + 1, CPU_CLK_EN_HI);
    clr8(mmio, REG_CPU_DMEM_CON + 2, WL_PLATFORM_RST_B2);
}

// Pulse the WL platform reset and 8051 clock so the reloaded firmware starts
// from a known state.
pub(super) fn reset_platform<M: Mmio>(mmio: &M) {
    clr8(mmio, REG_CPU_DMEM_CON + 2, WL_PLATFORM_RST_B2);
    clr8(mmio, REG_SYS_CLK_CTRL + 1, CPU_CLK_EN_HI);
    set8(mmio, REG_CPU_DMEM_CON + 2, WL_PLATFORM_RST_B2);
    set8(mmio, REG_SYS_CLK_CTRL + 1, CPU_CLK_EN_HI);
}

fn set8<M: Mmio>(mmio: &M, off: usize, bit: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, v | bit);
}

fn clr8<M: Mmio>(mmio: &M, off: usize, bit: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, v & !bit);
}
