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

//! The IVMD ranges, kept from boot for AMD-Vi bring-up.

use spin::Mutex;

use super::ivmd::{reserved, Reserved, MAX_RESERVED};

static RESERVED: Mutex<heapless::Vec<Reserved, MAX_RESERVED>> = Mutex::new(heapless::Vec::new());

pub(super) fn record(ivrs: &[u8]) {
    let none = Reserved { first: 0, last: 0, start: 0, end: 0, read: false, write: false };
    let mut found = [none; MAX_RESERVED];
    let n = reserved(ivrs, &mut found);
    *RESERVED.lock() = found[..n].iter().copied().collect();
}

/// The ranges firmware reserved for requester id `id`, in table order.
pub fn amd_reserved_for(id: u16) -> heapless::Vec<Reserved, MAX_RESERVED> {
    RESERVED.lock().iter().filter(|r| r.first <= id && id <= r.last).copied().collect()
}
