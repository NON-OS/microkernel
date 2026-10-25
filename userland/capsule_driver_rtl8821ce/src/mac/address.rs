// NONOS Operating System (AGPL-3.0-or-later)

//! The station address in the MAC-ID registers.

use crate::constants::regs::REG_MACID;
use crate::regs::Mmio;

/// Write the six-byte station MAC into the MAC-ID registers so the transmitter
/// stamps it as the source address and the receiver's physical-address filter
/// accepts unicast frames addressed to us.
pub fn set_address<M: Mmio>(mmio: &M, mac: &[u8; 6]) {
    mmio.write32(REG_MACID, u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]));
    mmio.write32(REG_MACID + 4, u32::from_le_bytes([mac[4], mac[5], 0, 0]));
}
