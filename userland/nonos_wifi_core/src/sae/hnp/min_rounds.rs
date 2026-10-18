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
//! The loop runs exactly 40 rounds, whatever the password, and treats every
//! round alike, so how many it took to find the element (which depends on
//! the password) shows neither in the time it takes nor in how many rounds
//! ran: once found, the remaining rounds hash a random stand-in password and
//! every choice is a constant-time select. This closes the Dragonblood
//! timing and cache side channels. A password with no element in those
//! rounds (a chance near 2^-40 for one pair of addresses) is refused rather
//! than hunted for longer; the station's next address for the network gives
//! it another pair. Checked against the IEEE Std 802.11-2020 Annex J.10
//! vector.

use nonos_p256::ProjectivePoint;

/// Rounds run whatever the password. Each round finds the element with
/// probability about one half on group 19.
pub(super) const ROUNDS: u8 = 40;

/// The longest password this loop takes, so the working copy needs no heap.
pub const PASSWORD_MAX: usize = 128;

/// The password element for two stations in `ROUNDS` rounds (`derive_pwe_in`).
/// `decoy` must be fresh random bytes as long as `password`: it is hashed in
/// the password's place once the element is found.
pub fn derive_pwe(
    password: &[u8],
    decoy: &[u8],
    a: &[u8; 6],
    b: &[u8; 6],
) -> Option<ProjectivePoint> {
    super::derive_pwe_in(password, decoy, a, b, ROUNDS)
}
