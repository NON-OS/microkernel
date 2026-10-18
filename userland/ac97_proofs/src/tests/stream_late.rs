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

//! A driver that looks late, and a queue that runs dry.

use super::feed::period;
use super::util::up;
use crate::constants::ring::{PERIOD_BYTES, RING_BYTES};
use crate::server::{refill, Stream};

// A driver held off for longer than the list lasts finds the engine halted
// on the last valid entry; its next look starts it again.
#[test]
fn a_late_look_restarts_a_halted_engine() {
    let (mut d, ich) = up();
    let mut st = Stream::default();
    assert!(st.start(&mut d));
    ich.borrow_mut().play(40 * PERIOD_BYTES);
    assert_eq!((ich.borrow().halts, ich.borrow().sr & 1), (1, 1));
    refill(&mut d, &mut st);
    assert_eq!(ich.borrow().sr & 1, 0, "running again");
    let before = ich.borrow().heard.len();
    ich.borrow_mut().play(PERIOD_BYTES);
    assert_eq!(ich.borrow().heard.len(), before + PERIOD_BYTES);
}

#[test]
fn a_queue_that_runs_dry_is_one_underrun_and_silence() {
    let (mut d, ich) = up();
    let mut st = Stream::default();
    assert!(st.start(&mut d));
    let mut fed = 0;
    for _ in 0..12 {
        period(&mut d, &mut st, &ich, &mut fed, 2 * PERIOD_BYTES);
    }
    assert_eq!((st.periods.played, st.periods.underruns), (2, 1));
    let lines = nonos_libc::logged();
    assert_eq!(lines.iter().filter(|l| l.contains("underrun")).count(), 1);
    let heard = &ich.borrow().heard;
    assert!(heard[RING_BYTES + 2 * PERIOD_BYTES..].iter().all(|&b| b == 0));
}
