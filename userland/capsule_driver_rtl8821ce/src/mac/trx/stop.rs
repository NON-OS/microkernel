// NONOS Operating System (AGPL-3.0-or-later)

//! The chip quiesced before anything it was given can be freed:
//! its transmit and receive DMA engines and the MAC's own stopped, then Bus
//! Master off, so no engine can write host memory after.

use super::regs::REG_CR;
use crate::regs::Mmio;

/// Stop every engine `init_trx_cfg` turned on (the whole of `REG_CR`'s low
/// byte), then turn Bus Master off through `bus_master_off`, which says
/// whether the write was taken.
pub fn quiesce<M: Mmio>(mmio: &M, bus_master_off: impl FnOnce() -> bool) -> bool {
    mmio.write8(REG_CR, 0);
    bus_master_off()
}
