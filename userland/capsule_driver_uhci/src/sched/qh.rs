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

//! A queue head's two words. Pure helpers over the region.

use crate::env::Mem;
use crate::facts::td::{LINK_QH, LINK_TERMINATE};

/// A link to the QH at `off`.
pub fn qh_link<M: Mem>(m: &M, off: usize) -> u32 {
    m.bus(off) | LINK_QH
}

/// The QH at `off` given its next QH (or the end) and no TDs.
pub fn init<M: Mem>(m: &M, off: usize, next: Option<usize>) {
    m.wr(off, next.map_or(LINK_TERMINATE, |n| qh_link(m, n)));
    m.wr(off + 4, LINK_TERMINATE);
}

/// Hand the queue the TD chain starting at `td`.
pub fn start<M: Mem>(m: &M, off: usize, td: usize) {
    m.wr(off + 4, m.bus(td));
}

/// Take the queue's TDs back: the controller runs none of them after this.
pub fn stop<M: Mem>(m: &M, off: usize) {
    m.wr(off + 4, LINK_TERMINATE);
}
