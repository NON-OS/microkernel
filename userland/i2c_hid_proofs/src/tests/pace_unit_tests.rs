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

//! How long the loop waits on its inbox: never 0, which waits for good,
//! and never past MAX_WAIT_MS.

use crate::drive::Pace;

#[test]
fn the_inbox_wait_is_at_least_a_millisecond_and_at_most_half_a_second() {
    let mut pace = Pace::default();
    assert_eq!(pace.inbox_wait_ms(5_000), 1, "a pass already due");
    pace.after(5_000, 0);
    assert_eq!(pace.inbox_wait_ms(5_000), 1, "a pass due now");
    pace.after(5_000, 7);
    assert_eq!(pace.inbox_wait_ms(5_000), 7);
    pace.after(5_000, 60_000);
    assert_eq!(pace.inbox_wait_ms(5_000), 500, "a pass far off");
}

#[test]
fn a_read_with_a_report_is_due_again_in_2_ms_and_a_quiet_one_in_16() {
    let mut pace = Pace::default();
    pace.read(10_000, true);
    assert!(!pace.due(10_001) && pace.due(10_002));
    pace.read(11_000, false);
    assert!(!pace.due(11_015) && pace.due(11_016), "quiet a second, not backed off");
}
