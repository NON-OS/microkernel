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

//! The vfs client's per-process deadline: 0 until a process sets one, so a
//! capsule that never does waits as it always has; a call that names its own
//! timeout keeps it; and the deadline once set bounds every other call.
//! One test, as the deadline is one setting of the whole process.

use crate::deadline::{effective, set_deadline_ms, TIMEOUT};

#[test]
fn the_deadline_is_zero_until_set_and_then_bounds_unnamed_calls() {
    // A capsule that never calls set_deadline_ms passes 0: the kernel's own.
    assert_eq!(effective(0), 0);
    assert_eq!(effective(150), 150, "a named timeout is the caller's own");
    set_deadline_ms(25);
    assert_eq!(effective(0), 25, "an unnamed call waits the process's deadline");
    assert_eq!(effective(150), 150, "a named timeout still wins");
    set_deadline_ms(0);
    assert_eq!(effective(0), 0, "set back to 0, the kernel's own again");
    for asked in [1, 25, 200, u64::MAX] {
        assert_eq!(effective(asked), asked);
    }
    assert_eq!(TIMEOUT, "vfs ipc failed", "the words callers already show");
}
