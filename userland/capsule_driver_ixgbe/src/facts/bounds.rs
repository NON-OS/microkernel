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

//! Every wait, in milliseconds. Where a value was seen in another driver it
//! says so and is unverified; where it is this driver's own, it says why.
//! The error lines that quote a bound are held to it in ixgbe_proofs.

/// Bus mastering must stop before reset: 80 ms observed in Linux ixgbe,
/// unverified.
pub const MASTER_DISABLE_MS: u64 = 80;
/// CTRL.RST clears itself about a millisecond after it is written (observed
/// in Linux ixgbe, unverified); three are allowed.
pub const RESET_CLEAR_MS: u64 = 3;
/// Settle after each reset: 50 ms observed in Linux ixgbe, unverified.
pub const RESET_SETTLE_MS: u64 = 50;
/// The EEPROM read after reset. This driver's own: waited on, not fatally,
/// so the queues are not set up while the part is still loading its
/// defaults.
pub const AUTO_READ_MS: u64 = 10;
/// The security block's receive path reporting idle: 40 ms observed in
/// Linux ixgbe, unverified.
pub const SECRX_MS: u64 = 40;
/// A queue enable or disable reading back: 20 ms observed in Linux ixgbe,
/// unverified.
pub const QUEUE_ENABLE_MS: u64 = 20;
