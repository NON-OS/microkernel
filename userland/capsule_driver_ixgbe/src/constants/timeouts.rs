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

//! How long each wait on the part may take. Linux counts passes with a
//! delay in each; the bound here is that count times the longest delay, so
//! this driver never gives up on a part sooner than Linux might. The error
//! strings that quote a bound are checked against these values in
//! ixgbe_proofs.

/// ixgbe_disable_pcie_primary: IXGBE_PCI_PRIMARY_DISABLE_TIMEOUT (800)
/// passes of udelay(100).
pub const MASTER_DISABLE_MS: u64 = 80;
/// ixgbe_reset_hw_82599 and _X540: usleep_range(1000, 1200) after the
/// reset write, then ten passes of udelay(1) for CTRL.RST to self-clear.
/// A millisecond of polling is allowed on top of the first.
pub const RESET_CLEAR_MS: u64 = 3;
/// The msleep(50) after each reset, "to allow time for any pending HW
/// events to complete".
pub const RESET_SETTLE_MS: u64 = 50;
/// IXGBE_EEC_ARD after the reset. Linux reads the EEPROM later, under its
/// semaphore, and does not wait on this bit in the reset; it is waited on
/// here, not fatally, so the queues are not set up while the part is still
/// loading its defaults.
pub const AUTO_READ_MS: u64 = 10;
/// ixgbe_disable_sec_rx_path_generic: IXGBE_MAX_SECRX_POLL (40) passes of
/// udelay(1000).
pub const SECRX_MS: u64 = 40;
/// Queue enable read-back: ixgbe_configure_tx_ring and _rx_ring wait ten
/// passes of usleep_range(1000, 2000).
pub const QUEUE_ENABLE_MS: u64 = 20;
