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

//! The class-specific descriptors that follow one interface: the CDC
//! functional descriptors (CDC 1.2, section 5.2.3), each as its bytes.

use alloc::vec::Vec;

use super::kinds::{CS_INTERFACE, INTERFACE};
use super::walk::walk;

/// The CS_INTERFACE descriptors after interface `number`, alternate 0,
/// up to the next interface descriptor.
pub fn functional(raw: &[u8], number: u8) -> Vec<&[u8]> {
    let mut inside = false;
    let mut out = Vec::new();
    for d in walk(raw) {
        if d[1] == INTERFACE && d.len() >= 4 {
            inside = d[2] == number && d[3] == 0;
        } else if inside && d[1] == CS_INTERFACE && d.len() >= 3 {
            out.push(d);
        }
    }
    out
}

/// The functional descriptor of `subtype` after interface `number`.
pub fn find_functional(raw: &[u8], number: u8, subtype: u8) -> Option<&[u8]> {
    functional(raw, number).into_iter().find(|d| d[2] == subtype)
}

/// The data interface a Union functional descriptor (subtype 0x06) names
/// for the communications interface `number`: its first subordinate.
pub fn union_data(raw: &[u8], number: u8) -> Option<u8> {
    find_functional(raw, number, 0x06).filter(|d| d.len() >= 5).map(|d| d[4])
}
