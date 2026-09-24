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

//! The VERSIONS cell, and picking the link version from it.

extern crate alloc;

use alloc::vec::Vec;

use crate::cell::{VarCell, CELL_VERSIONS};

use super::constants::LINK_VERSIONS;

/// Our VERSIONS cell, encoded ready to send.
pub fn offer() -> Vec<u8> {
    let mut body = Vec::with_capacity(LINK_VERSIONS.len() * 2);
    for version in LINK_VERSIONS {
        body.extend_from_slice(&version.to_be_bytes());
    }
    VarCell::new(CELL_VERSIONS, body).encode()
}

/// The highest version both ends offer, or `None` if there is no overlap.
///
pub fn negotiate(body: &[u8]) -> Option<u16> {
    let mut best = None;
    for pair in body.chunks_exact(2) {
        let offered = u16::from_be_bytes([pair[0], pair[1]]);
        if LINK_VERSIONS.contains(&offered) && Some(offered) > best {
            best = Some(offered);
        }
    }
    best
}
