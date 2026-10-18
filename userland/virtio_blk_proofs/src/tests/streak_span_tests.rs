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

//! What does not start the count of resets over: OK answers for less than
//! the span since the last reset, though the span from the first has
//! passed; answers the device failed; and OK answers while the clock
//! fails, which can show no span served.

use nonos_libc::break_clock;

use super::served::up;
use super::streak_tests::GONE;
use crate::constants::{VIRTIO_BLK_S_IOERR, VIRTIO_BLK_S_OK};
use crate::server::streak::{RESETS_IN_A_ROW, SERVED_SPAN_MS};

#[test]
fn a_device_that_serves_less_than_the_span_since_its_last_reset_is_given_up() {
    let mut up = up();
    assert_eq!(up.turns(RESETS_IN_A_ROW), None);
    // The resets came a read's budget apart, so the span since the first
    // of them has passed by the last answer.
    up.answers_for(SERVED_SPAN_MS - 1, VIRTIO_BLK_S_OK);
    assert_eq!(up.turns(1).as_deref(), GONE);
}

#[test]
fn a_device_that_fails_what_it_answers_is_not_serving() {
    let mut up = up();
    assert_eq!(up.turns(RESETS_IN_A_ROW), None);
    up.answers_for(2 * SERVED_SPAN_MS, VIRTIO_BLK_S_IOERR);
    assert_eq!(up.turns(1).as_deref(), GONE);
}

#[test]
fn a_failed_clock_never_starts_the_count_over() {
    let mut up = up();
    assert_eq!(up.turns(RESETS_IN_A_ROW), None);
    break_clock(true);
    up.answers_for(2 * SERVED_SPAN_MS, VIRTIO_BLK_S_OK);
    break_clock(false);
    assert_eq!(up.turns(1).as_deref(), GONE);
}
