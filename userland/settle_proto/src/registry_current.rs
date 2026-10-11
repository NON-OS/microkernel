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

//! What a listing has now, from its registry logs: every Published for it,
//! less any release a Withdrawn names (for good: a withdrawn release is not
//! listed again, so the order of the two log reads cannot matter), one row
//! per release (its newest Published), newest first, at most LISTED_MAX.

use alloc::vec::Vec;

use crate::registry_log::Listed;

pub const LISTED_MAX: usize = 16;

pub fn current(
    listing: &[u8; 32],
    published: &[Listed],
    withdrawn: &[([u8; 32], [u8; 32])],
) -> Vec<Listed> {
    let mut out: Vec<Listed> = Vec::new();
    let gone = |l: &Listed| withdrawn.iter().any(|(li, r)| li == listing && *r == l.release_id);
    for l in published.iter().filter(|l| l.listing_id == *listing && !gone(l)) {
        match out.iter_mut().find(|o| o.release_id == l.release_id) {
            Some(o) if l.block >= o.block => *o = l.clone(),
            Some(_) => {}
            None => out.push(l.clone()),
        }
    }
    out.sort_by(|a, b| b.block.cmp(&a.block));
    out.truncate(LISTED_MAX);
    out
}
