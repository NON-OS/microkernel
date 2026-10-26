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

//! The host reading and writing the grant by the page numbers it was handed.

use super::{Grant, PAGE};

pub(super) fn gather(g: Grant, pfns: &[u64], len: usize) -> Vec<u8> {
    let mut v = vec![0u8; len];
    for (i, chunk) in v.chunks_mut(PAGE).enumerate() {
        // SAFETY: a page of the grant (Grant::at_pfn checks).
        unsafe {
            std::ptr::copy_nonoverlapping(g.at_pfn(pfns[i]), chunk.as_mut_ptr(), chunk.len())
        };
    }
    v
}

pub(super) fn scatter(g: Grant, pfns: &[u64], data: &[u8]) {
    for (i, chunk) in data.chunks(PAGE).enumerate() {
        // SAFETY: as above.
        unsafe { std::ptr::copy_nonoverlapping(chunk.as_ptr(), g.at_pfn(pfns[i]), chunk.len()) };
    }
}
