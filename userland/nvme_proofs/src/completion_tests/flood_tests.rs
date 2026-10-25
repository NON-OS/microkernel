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

//! A hostile controller that floods the ring with stray entries cannot keep
//! a wait from ending on time.

use core::cell::Cell;

use super::ring::{entry, ok_status, RINGS};
use crate::admin::{wait_for_completion, CqCursor, DEADLINE_CHECK_SPINS};
use crate::error::NvmeError;

#[test]
fn a_controller_flooding_stray_entries_still_ends_the_wait() {
    // A hostile controller writes a fresh stray entry, with the phase the
    // driver expects, into every slot the driver reaches. The driver
    // consumes each and still gives up on time.
    for entries in RINGS {
        for allowed in [0u64, 3] {
            let shadow = Cell::new((0u16, true));
            let (reads, checks, rung) = (Cell::new(0u64), Cell::new(0u64), Cell::new(0u64));
            let mut c = CqCursor::new(entries);
            let result = wait_for_completion(
                &mut c,
                0,
                5,
                |head| {
                    reads.set(reads.get() + 1);
                    assert!(reads.get() < 50_000_000, "the completion wait did not end");
                    assert_eq!(head, shadow.get().0);
                    entry(6, 0, 0, ok_status(shadow.get().1))
                },
                |head| {
                    assert!(head < entries);
                    rung.set(rung.get() + 1);
                    let (_, phase) = shadow.get();
                    shadow.set((head, if head == 0 { !phase } else { phase }));
                },
                || {
                    checks.set(checks.get() + 1);
                    checks.get() > allowed
                },
            );
            assert!(matches!(result, Err(NvmeError::ControllerTimeout)));
            assert_eq!(checks.get(), allowed + 1);
            assert_eq!(
                reads.get(),
                (allowed + 1) * DEADLINE_CHECK_SPINS as u64 + 1,
                "and a last look"
            );
            assert_eq!(rung.get(), reads.get(), "every stray entry is consumed");
        }
    }
}
