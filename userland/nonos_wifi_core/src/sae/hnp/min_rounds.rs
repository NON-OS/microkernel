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

//! SAE hunting and pecking (IEEE Std 802.11-2020, 12.4.4.2.2), for an access
//! point that does not advertise hash-to-element:
//!
//!   pwd-seed  = HMAC-SHA256(MAX(MAC) || MIN(MAC), password || counter)
//!   pwd-value = KDF-SHA256-256(pwd-seed, "SAE Hunting and Pecking", p)
//!
//! and the first counter whose pwd-value is below p and an x-coordinate on
//! the curve gives the PWE, with y chosen so its parity matches pwd-seed's.
//!
//! The loop always runs at least 40 rounds and treats every round alike, so
//! how many it took to find the element (which depends on the password) does
//! not show in the time it takes: once found, the remaining rounds hash a
//! random stand-in password and every choice is a constant-time select. This
//! closes the Dragonblood timing and cache side channels. Checked against
//! the IEEE Std 802.11-2020 Annex J.10 vector.

/// Rounds run whatever the password. Each round finds the element with
/// probability about one half on group 19, so after 40 the chance a password
/// still needs more is near 2^-40.
pub(super) const MIN_ROUNDS: u8 = 40;

/// A bound on the loop: the chance of no element in 200 rounds is about 2^-200.
pub(super) const MAX_ROUNDS: u8 = 200;

/// The longest password this loop takes, so the working copy needs no heap.
pub const PASSWORD_MAX: usize = 128;
