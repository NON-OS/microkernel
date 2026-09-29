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

//! Taking address space for a guest without backing it.

use super::handle::Guest;
use super::layout::USER_MAX;
use super::mem::span_within;
use super::region::Region;

impl Guest {
    /// Take `len` of address space at `addr` without backing it: a PROT_NONE
    /// reservation. No page exists until a commit maps one; a touch before
    /// that is a fault, as it is on Linux.
    pub fn reserve(&mut self, addr: u64, len: u64) -> i64 {
        let Some((start, span)) = span_within(addr, len, USER_MAX) else {
            return -1;
        };
        self.regions.push(Region {
            at: start,
            len: span,
            write: false,
            exec: false,
            access: false,
            unproven: false,
            backed: false,
        });
        0
    }
}
