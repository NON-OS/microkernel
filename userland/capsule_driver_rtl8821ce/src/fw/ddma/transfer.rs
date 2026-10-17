// NONOS Operating System (AGPL-3.0-or-later)

//! One DDMA transfer into the 8051's memory, and the channel's idle wait.

use crate::regs::Mmio;

use super::regs::OWN_MS;
use super::{
    DDMA_CHKSUM_CONT, DDMA_CHKSUM_EN, DDMA_DLEN_MASK, DDMA_OWN, DDMA_RESET_CHKSUM_STS, MCUFWDL_EN,
    REG_DDMA_CH0CTRL, REG_DDMA_CH0DA, REG_DDMA_CH0SA, REG_MCUFW_CTRL,
};
use crate::wait::until_ms;

/// Enable firmware download: set the download-enable bit in the firmware
/// control register while preserving the reserved bits the card owns.
pub fn enable_download<M: Mmio>(mmio: &M) {
    let cur = mmio.read16(REG_MCUFW_CTRL) & 0x3800;
    mmio.write16(REG_MCUFW_CTRL, cur | MCUFWDL_EN);
}

/// Reset the running checksum before the first chunk of a section.
pub fn reset_checksum<M: Mmio>(mmio: &M) {
    let cur = mmio.read32(REG_DDMA_CH0CTRL);
    mmio.write32(REG_DDMA_CH0CTRL, cur | DDMA_RESET_CHKSUM_STS);
}

/// Copy one chunk: `src` is the packet-buffer address, `dst` the on-chip
/// target, `len` the byte count (18-bit). `first` clears the continuation bit
/// so the section's checksum starts fresh; later chunks accumulate. Returns
/// false if the channel never releases ownership (a dead card).
pub fn transfer<M: Mmio>(mmio: &M, src: u32, dst: u32, len: u32, first: bool) -> bool {
    if !wait_idle(mmio) {
        return false;
    }
    let mut ctrl = DDMA_CHKSUM_EN | DDMA_OWN | (len & DDMA_DLEN_MASK);
    if !first {
        ctrl |= DDMA_CHKSUM_CONT;
    }
    mmio.write32(REG_DDMA_CH0SA, src);
    mmio.write32(REG_DDMA_CH0DA, dst);
    mmio.write32(REG_DDMA_CH0CTRL, ctrl);
    wait_idle(mmio)
}

// The channel-owned bit is set by the host to start a transfer and cleared by
// the card when it finishes; a transfer may only start when it reads clear.
fn wait_idle<M: Mmio>(mmio: &M) -> bool {
    until_ms(OWN_MS, || mmio.read32(REG_DDMA_CH0CTRL) & DDMA_OWN == 0)
}
