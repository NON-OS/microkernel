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

//! capsule_acpi's power/retry.rs: a reading owed is due 250 ms on, the wait
//! doubling, eight times; an answer clears it.

use super::retry::{Retry, FIRST_MS, TRIES};

#[test]
fn an_owed_reading_backs_off_and_ends() {
    let mut r = Retry::default();
    assert!(!r.due(u64::MAX), "nothing owed");
    let mut now = 1_000;
    for i in 0..TRIES {
        r.after(true, now);
        let at = now + (FIRST_MS << i);
        assert!(!r.due(at - 1) && r.due(at), "try {i}");
        now = at;
    }
    r.after(true, now);
    assert!(!r.due(u64::MAX), "after eight, the minute's poll alone");
    r.after(false, now);
    r.after(true, now);
    assert!(r.due(now + FIRST_MS), "an answer starts it over");
}
