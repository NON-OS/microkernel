// NONOS Operating System (AGPL-3.0-or-later)

//! Whether the card is still on the bus.

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::constants::regs::REG_SYS_CFG1;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// True once the card reads back all ones: a read nothing on the bus
    /// answered, as after a surprise removal or a link that went down. The
    /// system configuration register holds the chip's id and never reads so
    /// from a live card.
    pub fn card_gone(&self) -> bool {
        self.mmio.read32(REG_SYS_CFG1) == u32::MAX
    }
}
