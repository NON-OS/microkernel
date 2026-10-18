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

//! The claim on the VMBus root, recorded with every other broker call.

use std::cell::Cell;

use crate::script::{answer, note};

thread_local! {
    pub(crate) static CLAIMED: Cell<Option<u64>> = const { Cell::new(None) };
}

pub fn mk_device_claim(device_id: u64) -> i64 {
    note("claim", device_id);
    let rc = answer().claim;
    if rc >= 0 {
        CLAIMED.with(|c| c.set(Some(device_id)));
    }
    rc
}

pub fn mk_device_release(device_id: u64) -> i64 {
    note("device_release", device_id);
    CLAIMED.with(|c| c.set(None));
    0
}
