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

//! The live DMA grants of this thread, each backed by host memory that is
//! never freed, so a device access after the grant goes is caught, not UB.

use std::cell::RefCell;

pub(crate) struct Grant {
    pub(crate) id: u64,
    pub(crate) device: u64,
    pub(crate) iova: u64,
    pub(crate) ptr: *mut u8,
    pub(crate) len: usize,
    pub(crate) live: bool,
}

thread_local! {
    pub(crate) static GRANTS: RefCell<Vec<Grant>> = const { RefCell::new(Vec::new()) };
}

/// What MkDeviceRelease does to the grants: every one of the device goes.
pub(crate) fn drain(device: u64) {
    GRANTS.with(|g| {
        g.borrow_mut().iter_mut().filter(|x| x.device == device).for_each(|x| x.live = false)
    });
}

/// The grants a test can still read through: those not yet torn down.
pub fn live_grants() -> usize {
    GRANTS.with(|g| g.borrow().iter().filter(|x| x.live).count())
}
