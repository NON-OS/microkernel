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
//! The SD Host Controller specification sets none of these bounds as
//! recalled; each says where its value comes from.

/// A software reset bit clears within this: 100 ms, as observed,
/// unverified.
pub const RESET_MS: u64 = 100;
/// The internal clock reports stable within this: 150 ms, as observed,
/// unverified.
pub const CLOCK_STABLE_MS: u64 = 150;
/// CMD and DAT inhibit clear within this before a command is sent. This
/// engine's own: a previous command's busy phase has its own bound first.
pub const INHIBIT_MS: u64 = 100;
/// Command Complete follows the command within this. This engine's own: a
/// 136-bit response at 400 kHz takes under half a millisecond.
pub const CMD_MS: u64 = 250;
/// Supply and clock settle after each is turned on: 10 ms, as observed,
/// unverified; it also covers the 74 clocks a card needs before its first
/// command at 400 kHz many times over.
pub const SETTLE_MS: u64 = 10;
/// Card detect debounces within this after a full reset. This engine's own.
pub const DETECT_MS: u64 = 100;
/// Bus power writes before the bit is taken as refused.
pub const POWER_TRIES: usize = 3;
pub const BLOCK: usize = 512;
