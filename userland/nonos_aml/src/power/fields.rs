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

//! Reading a battery package's elements: integers where 0xFFFFFFFF means
//! unknown (ACPI 6.5, 10.2.2), and strings that firmware sometimes gives as
//! buffers.

use crate::object::{Object, Pkg};
use alloc::vec::Vec;

pub(super) const UNKNOWN: u64 = 0xFFFF_FFFF;

pub(super) fn elements(o: &Object) -> Option<Vec<Object>> {
    match o {
        Object::Package(p) => Some(Pkg::clone(p).borrow().clone()),
        _ => None,
    }
}

/// An integer element, None when it is missing, not an integer, or unknown.
pub(super) fn known(e: &[Object], i: usize) -> Option<u32> {
    match e.get(i)? {
        Object::Integer(v) if *v & UNKNOWN != UNKNOWN => u32::try_from(*v).ok(),
        _ => None,
    }
}

/// A string element, up to its NUL; empty when it is anything else.
pub(super) fn text(e: &[Object], i: usize) -> Vec<u8> {
    match e.get(i) {
        Some(Object::String(s) | Object::Buffer(s)) => {
            s.borrow().iter().copied().take_while(|b| *b != 0).take(64).collect()
        }
        _ => Vec::new(),
    }
}
