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

//! A completion ring handed to the controller must hold no phase tag. The
//! driver's cursor starts at slot 0 waiting for phase 1, so an entry left
//! from an earlier pass, its ring not zeroed before the controller took it
//! again, would pass for a new completion. The fake checks every ring as it
//! takes it: the admin ring at CC.EN 0 to 1, the I/O ring at Create I/O CQ.

use core::ptr::read_volatile;

use crate::admin::Completion;

/// Bytes of a completion ring: one page, as the driver maps each.
const RING_BYTES: u64 = 4096;

/// Fail the test unless every one of the `entries` slots at `cq` has phase 0.
pub(super) fn unwritten(cq: u64, entries: u16, what: &str) {
    assert!(cq != 0, "{what} was never named");
    assert!(u64::from(entries) * 16 <= RING_BYTES, "{what} is past its page");
    for slot in 0..u64::from(entries) {
        // SAFETY: the driver's own completion ring, a page of host memory
        // from the shim, and the slot lies inside it (the assert above).
        let c = unsafe { read_volatile((cq + slot * 16) as *const Completion) };
        assert!(!c.phase(), "{what} holds a phase tag at slot {slot} as it is taken");
    }
}
