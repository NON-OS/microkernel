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

//! The RMRRs DMAR listed, as parsed. A structure that does not parse is
//! dropped and said: its devices get no reserved range, the safe side.

use spin::Mutex;

use super::dmar_rmrr::{parse_rmrr, Reserved};

pub(crate) const MAX_RESERVED: usize = 16;

static RESERVED: Mutex<heapless::Vec<Reserved, MAX_RESERVED>> = Mutex::new(heapless::Vec::new());

pub(super) fn record(bytes: &[u8]) {
    let kept = parse_rmrr(bytes).is_some_and(|r| RESERVED.lock().push(r).is_ok());
    if !kept {
        crate::sys::serial::println(b"[VT-D] reserved memory region refused or over the cap");
    }
}

/// The reserved memory regions firmware listed, in DMAR order.
pub fn reserved_regions() -> heapless::Vec<Reserved, MAX_RESERVED> {
    RESERVED.lock().clone()
}
