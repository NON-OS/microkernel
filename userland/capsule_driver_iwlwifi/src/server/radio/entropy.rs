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
use nonos_wifi_core::mlme::{Entropy, SAE_ENTROPY};

/// The join's randomness, or `None` if the kernel gave none.
pub fn draw_entropy() -> Option<Entropy> {
    let mut e = Entropy { snonce: [0u8; 32], sae: [0u8; SAE_ENTROPY] };
    let a = crypto_random(e.snonce.as_mut_ptr(), e.snonce.len());
    let b = crypto_random(e.sae.as_mut_ptr(), e.sae.len());
    (a == e.snonce.len() as i64 && b == e.sae.len() as i64).then_some(e)
}
