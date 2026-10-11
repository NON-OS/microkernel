// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! ixgbe_disable_rx_queue: the enable cleared and seen cleared before the
//! queue's base or length change under it.

use crate::constants::regs::REG_RXDCTL;
use crate::constants::rx_bits::RXDCTL_ENABLE;
use crate::constants::timeouts::QUEUE_ENABLE_MS;
use crate::init::wait::until;
use crate::regs::Regs;

/// Quotes QUEUE_ENABLE_MS; ixgbe_proofs holds the two equal.
pub const RX_NOT_DISABLED: &str = "rx queue disable did not read back in 20 ms";

/// RXDCTL without the enable, the rest as found.
pub fn disable(regs: &Regs) -> Result<u32, &'static str> {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and RXDCTL
    // is a 32-bit register inside it (ixgbe_type.h).
    let rxdctl = unsafe { regs.r32(REG_RXDCTL) } & !RXDCTL_ENABLE;
    // SAFETY: as above.
    unsafe { regs.w32(REG_RXDCTL, rxdctl) };
    // SAFETY: as above.
    let off = || unsafe { regs.r32(REG_RXDCTL) } & RXDCTL_ENABLE == 0;
    if until(QUEUE_ENABLE_MS, off) {
        Ok(rxdctl)
    } else {
        Err(RX_NOT_DISABLED)
    }
}
