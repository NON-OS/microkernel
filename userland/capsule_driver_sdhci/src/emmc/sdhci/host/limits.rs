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

//! How long each step of the host may take, and the block size it moves.

/// A software reset bit clears within this (Linux sdhci_reset: 100 ms).
pub const RESET_MS: u64 = 100;
/// The internal clock reports stable within this (Linux: 150 ms).
pub const CLOCK_STABLE_MS: u64 = 150;
/// CMD and DAT inhibit clear within this before a command is sent.
pub const INHIBIT_MS: u64 = 100;
/// Command Complete follows the command within this. A 136-bit response at
/// 400 kHz takes under half a millisecond.
pub const CMD_MS: u64 = 250;
/// Supply and clock settle after each is turned on (Linux's
/// power_delay_ms, waited after power up and again after the first clock).
pub const SETTLE_MS: u64 = 10;
/// Card detect debounces within this after a full reset.
pub const DETECT_MS: u64 = 100;
pub const BLOCK: usize = 512;
