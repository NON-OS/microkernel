// NONOS Operating System (AGPL-3.0-or-later)

//! The beacon-valid wait that says a staged page landed.

use super::super::regs::{BCN_VALID_V1, REG_FIFOPAGE_CTRL_2};
use crate::regs::Mmio;
use crate::wait::until_ms;

/// How long a reserved-page write may take to land (rtw88 `check_hw_ready`).
const BCN_VALID_MS: u64 = 10;

// The card sets beacon-valid once the staged page has landed in the packet
// buffer; a page that never validates means a wedged or unpowered card.
pub(super) fn wait_bcn_valid<M: Mmio>(mmio: &M) -> bool {
    until_ms(BCN_VALID_MS, || mmio.read16(REG_FIFOPAGE_CTRL_2) & BCN_VALID_V1 != 0)
}
