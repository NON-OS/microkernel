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

//! This capsule's table of the proofs its current guest family has had
//! verified (attest_seen_rule.rs says why an answer may be kept).

use core::cell::RefCell;

#[path = "attest_seen_rule.rs"]
mod rule;
#[path = "attest_seen_key.rs"]
mod seen_key;

use rule::Seen;
pub use seen_key::key;

use nonos_libc::CapsuleVerifySummary;

struct Table(RefCell<Seen<CapsuleVerifySummary>>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that proves a
 * program; guest threads run in their own processes and only trap into it.
 */
unsafe impl Sync for Table {}

static SEEN: Table = Table(RefCell::new(Seen::new()));

pub fn known(key: &[u8; 32]) -> Option<CapsuleVerifySummary> {
    SEEN.0.borrow().known(key)
}

pub fn accepted(key: [u8; 32], summary: CapsuleVerifySummary) {
    SEEN.0.borrow_mut().accepted(key, summary);
}

/// Called before every mk_foreign_spawn that may begin a family: the kernel
/// reserves a new family's seal at a supervisor's first guest
/// (src/process/foreign/seal/open.rs), and that seal must record its own.
pub fn forget() {
    SEEN.0.borrow_mut().forget();
}
