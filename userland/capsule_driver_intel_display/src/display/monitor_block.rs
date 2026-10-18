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

//! One EDID block, taken only whole. It is read up to TRIES times while
//! the whole read's deadline lasts, and kept the first time it reads as a
//! block: its checksum holds (E-EDID 1.4 section 3.11), the base starts
//! with the header, and an extension is none of the blocks before it,
//! which is what a sink ignoring its index or its segment pointer hands
//! back. Each try lands in its own buffer, so a cut, shifted or repeated
//! read never reaches the blob.

use super::monitor_try::once;
use crate::engine::access::{Clock, Expiry, Mmio};
use crate::engine::gmbus::BLOCK;
use crate::engine::gmbus_xfer::deadline::Deadline;
use crate::engine::route::Route;
use nonos_edid::blocks::{checksum_ok, HEADER};

pub const TRIES: usize = 3;

pub fn block(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    r: &Route,
    n: u8,
    before: &[u8],
    whole: &Deadline,
) -> Option<[u8; BLOCK]> {
    for _ in 0..TRIES {
        if whole.expired() {
            return None;
        }
        let mut b = [0u8; BLOCK];
        if once(m, clock, r, n, &mut b, whole).is_some() && fits(&b, before) {
            return Some(b);
        }
    }
    None
}

fn fits(b: &[u8; BLOCK], before: &[u8]) -> bool {
    let fresh = match before.is_empty() {
        true => b[..8] == HEADER,
        false => !before.chunks(BLOCK).any(|p| p == b),
    };
    checksum_ok(b) && fresh
}
