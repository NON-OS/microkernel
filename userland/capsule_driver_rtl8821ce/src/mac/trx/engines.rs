// NONOS Operating System (AGPL-3.0-or-later)

//! The engines switched on: queue mapping, the page boundaries, the link-list table.

use super::bounds::txdma_pq_map;
use super::pages::{AUTO_INIT_LLT_V1, H2CQ_FULL};
use super::queues::{init_h2c, priority_queue_cfg};
use super::regs::{MAC_TRX_ENABLE, REG_AUTO_LLT_V1, REG_CR, REG_H2CQ_CSR, REG_TXDMA_PQ_MAP};
use crate::regs::Mmio;
use crate::wait::until_ms;

/// How long the link-list-table auto-init may take (rtw88 `check_hw_ready`).
const LLT_MS: u64 = 10;

/// Configure and enable the MAC's transmit and receive engines. Returns false if
/// the link-list table never finished initialising or the H2C queue did not come
/// up with its whole ring free, either of which means the MAC is not usable.
pub fn init_trx_cfg<M: Mmio>(mmio: &M) -> bool {
    queue_mapping(mmio);
    if !priority_queue_cfg(mmio) {
        return false;
    }
    init_h2c(mmio)
}

// txdma_queue_mapping: map the priorities onto the DMA queues, then bracket the
// map write with clearing and setting REG_CR so all eight engine bits come up.
fn queue_mapping<M: Mmio>(mmio: &M) {
    mmio.write16(REG_TXDMA_PQ_MAP, txdma_pq_map());
    mmio.write8(REG_CR, 0);
    mmio.write8(REG_CR, MAC_TRX_ENABLE);
    mmio.write32(REG_H2CQ_CSR, H2CQ_FULL);
}

// Wait for the auto-init link-list-table bit to clear.
pub(super) fn poll_llt<M: Mmio>(mmio: &M) -> bool {
    until_ms(LLT_MS, || mmio.read8(REG_AUTO_LLT_V1) & AUTO_INIT_LLT_V1 == 0)
}
