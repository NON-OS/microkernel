// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Every capsule serving the block wire, in the order the block layer asks
//! them. A new block driver appends its mirror's endpoint here.

use core::sync::atomic::AtomicU64;

use super::endpoint::Endpoint;

static DRIVERS: &[&Endpoint] = &[];

/// How many endpoints this kernel carries.
pub fn count() -> u8 {
    DRIVERS.len() as u8
}

pub(super) fn endpoint(index: u8) -> Option<&'static Endpoint> {
    DRIVERS.get(index as usize).copied()
}

/// What the log calls endpoint `index`'s disk.
pub fn shown(index: u8) -> &'static str {
    endpoint(index).map_or("an unknown block-wire driver", |e| e.shown)
}

/// Where the block layer keeps the hash of its last line for endpoint
/// `index`. Past the table (never asked: the order only counts to `count`)
/// the lines share one slot.
pub fn said(index: u8) -> &'static AtomicU64 {
    static PAST: AtomicU64 = AtomicU64::new(0);
    endpoint(index).map_or(&PAST, |e| &e.slot.said)
}
