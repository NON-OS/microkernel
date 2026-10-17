// NONOS Operating System (AGPL-3.0-or-later)

//! The beacon-valid wait that says a staged page landed.

use super::super::regs::{BCN_VALID_V1, REG_FIFOPAGE_CTRL_2};
use crate::regs::Mmio;

/// Reads allowed before the reserved-page write is declared not to have landed.
const BCN_VALID_POLL_LIMIT: u32 = 1_000_000;

// The card sets beacon-valid once the staged page has landed in the packet
// buffer; a page that never validates means a wedged or unpowered card.
pub(super) fn wait_bcn_valid<M: Mmio>(mmio: &M) -> bool {
    for _ in 0..BCN_VALID_POLL_LIMIT {
        if mmio.read16(REG_FIFOPAGE_CTRL_2) & BCN_VALID_V1 != 0 {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}
