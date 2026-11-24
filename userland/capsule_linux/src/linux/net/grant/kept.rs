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

//! The policy the run's program was bound to, kept when it is proved
//! (start_guest.rs), for the grant asked by policy. Only the run's own
//! program is kept: GTK's tools are not the run.

use core::cell::RefCell;

use nonos_linux_policy::LEN;

struct Kept(RefCell<Option<[u8; LEN]>>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that proves a
 * program or opens a stream; guest threads run in their own processes.
 */
unsafe impl Sync for Kept {}

static POLICY: Kept = Kept(RefCell::new(None));

/// Keep `policy`, the run's program's bound policy in its canonical bytes.
pub fn keep(policy: [u8; LEN]) {
    *POLICY.0.borrow_mut() = Some(policy);
}

pub(super) fn policy() -> Option<[u8; LEN]> {
    *POLICY.0.borrow()
}
