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

//! NBTC v1's operations, and the status words its replies carry. The
//! payloads are in look.rs, list.rs, part.rs and body.rs; the table is in
//! letter E-25.

/// Version and whether a controller is up. Settings and the shell.
pub const OP_HEALTH: u16 = 1;
/// The link's state and what it carries. Settings and the shell.
pub const OP_LINK: u16 = 2;
/// Battery levels, by bond. Settings and the shell.
pub const OP_BATTERY: u16 = 3;
/// The bonds, with identity addresses and never keys. Settings only.
pub const OP_BONDS: u16 = 4;
/// Forget one bond, ending its link. Settings only.
pub const OP_FORGET: u16 = 5;
/// Let one new device pair, for `PAIR_MS`. Settings only.
pub const OP_PAIR: u16 = 6;
/// What pairing asks of the user now. Settings only.
pub const OP_PASSKEY: u16 = 7;
/// The user's answer to it. Settings only.
pub const OP_ANSWER: u16 = 8;
/// Scanning on or off. Settings only.
pub const OP_SCAN: u16 = 9;

pub const E_OK: i32 = 0;
pub const E_NOENT: i32 = -2;
pub const E_IO: i32 = -5;
pub const E_ACCES: i32 = -13;
pub const E_BUSY: i32 = -16;
pub const E_INVAL: i32 = -22;

/// How long a pairing window stays open (decision 2 of letter E-25).
pub const PAIR_MS: u64 = 60_000;

/// OP_LINK's state octet.
pub const NO_CONTROLLER: u8 = 0;
pub const IDLE: u8 = 1;
pub const SCANNING: u8 = 2;
pub const CONNECTING: u8 = 3;
pub const SECURING: u8 = 4;
pub const READY: u8 = 5;

/// OP_PASSKEY's and OP_ANSWER's kind octet.
pub const PART_NONE: u8 = 0;
pub const PART_SHOW: u8 = 1;
pub const PART_ENTER: u8 = 2;
pub const PART_CONFIRM: u8 = 3;
