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

//! The driver's completion wait, run over a scripted ring.

use core::cell::{Cell, RefCell};

use crate::admin::{wait_for_completion, Completion, CqCursor};
use crate::error::NvmeResult;

pub(crate) struct Waited {
    pub result: NvmeResult<()>,
    pub reads: u64,
    pub rung: Vec<u16>,
    pub checks: u64,
}

/// Run the driver's completion wait for command `cid` on submission queue
/// `sq_id` over `ring` (what the controller has written), with the deadline
/// passing after `checks_allowed` checks. Every slot read and doorbell value
/// is checked against the ring as it happens.
pub(crate) fn wait_on(
    cursor: &mut CqCursor,
    sq_id: u16,
    cid: u16,
    ring: &[Completion],
    checks_allowed: u64,
) -> Waited {
    let entries = cursor.entries;
    assert_eq!(ring.len(), entries as usize);
    let reads = Cell::new(0u64);
    let checks = Cell::new(0u64);
    let rung = RefCell::new(Vec::new());
    let result = wait_for_completion(
        cursor,
        sq_id,
        cid,
        |head| {
            assert!(head < entries, "read slot {head} of a {entries}-entry ring");
            reads.set(reads.get() + 1);
            assert!(reads.get() < 50_000_000, "the completion wait did not end");
            ring[head as usize]
        },
        |head| {
            assert!(head < entries, "rang head {head} of a {entries}-entry ring");
            rung.borrow_mut().push(head);
        },
        || {
            checks.set(checks.get() + 1);
            assert!(checks.get() < 1_000_000, "the completion wait did not end");
            checks.get() > checks_allowed
        },
    );
    Waited { result, reads: reads.get(), rung: rung.into_inner(), checks: checks.get() }
}
