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

//! Fresh randomness for one join, drawn from the kernel as the RTL8821CE
//! driver draws it: the SNonce (sent in message 2) and, apart from it, SAE's
//! rand, mask and stand-in password, so nothing secret is derived from a
//! value that goes on the air.

use nonos_libc::crypto_random;
use nonos_wifi_core::mlme::Entropy;

/// The join's randomness, or `None` if the kernel gave none. A connect draws
/// it first, so without it the join is refused before the current network is
/// left or a frame goes out.
pub fn draw_entropy() -> Option<Entropy> {
    Entropy::draw(|b| crypto_random(b.as_mut_ptr(), b.len()) == b.len() as i64)
}
