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

//! Fresh randomness for a join.

use nonos_libc::crypto_random;
use nonos_wifi_core::mlme::Entropy;
use nonos_wifi_core::vault::wipe;

use crate::fw::dma::Grant;
use crate::link::RtlLink;
use crate::regs::Regs;

// Fresh randomness for one join, or `None` if the kernel gave none.
pub(super) fn draw_entropy() -> Option<Entropy> {
    Entropy::draw(|b| crypto_random(b.as_mut_ptr(), b.len()) == b.len() as i64)
}

/// A fresh seed for the next pairwise rekey's SNonce, taken once a rekey has
/// finished. Without one the station keeps its seed; its count of rekeys
/// still keeps every SNonce new.
pub(in crate::serve) fn reseed(link: &mut RtlLink<Regs, Grant, Grant>) {
    let mut seed = [0u8; 32];
    if crypto_random(seed.as_mut_ptr(), seed.len()) == seed.len() as i64 {
        link.reseed_rekey(seed);
    }
    wipe(&mut seed);
}
