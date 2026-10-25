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

//! A device gone mid-stream is noticed (the lead's review, R16). Each
//! failed batch is followed by one hollow success from driver.xhci0, and
//! that pattern must still count up to LOST_AFTER; two real successes in a
//! row clear the count.

use crate::serve::loss::{Loss, LOST_AFTER};

#[test]
fn failures_each_followed_by_a_hollow_success_still_count() {
    let mut l = Loss::default();
    let mut gone = false;
    for _ in 0..LOST_AFTER {
        gone = l.note(false);
        if !gone {
            assert!(!l.note(true), "the success after an error is not news");
        }
    }
    assert!(gone, "a device that fails every other batch is gone");
}

#[test]
fn two_successes_in_a_row_clear_the_count() {
    let mut l = Loss::default();
    for _ in 0..LOST_AFTER - 1 {
        assert!(!l.note(false));
        assert!(!l.note(true));
    }
    assert!(!l.note(true), "a success after a success");
    for _ in 0..LOST_AFTER - 1 {
        assert!(!l.note(false));
        assert!(!l.note(true));
    }
    assert!(l.note(false), "counted afresh to LOST_AFTER");
    l.clear();
    assert_eq!(l, Loss::default());
}
