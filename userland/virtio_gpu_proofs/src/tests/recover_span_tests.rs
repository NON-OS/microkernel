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

//! What starts the count of recoveries in a row over: serving SERVED_MS by
//! the clock since the last one, not an answer. A device that answers one
//! present per recovery and stalls the next started it over with every
//! answer, and was recovered forever, reporting Live; it is `Down` after
//! IN_A_ROW of them now, in bounded time, even after serving long before it
//! began to stall. One that serves through the span after each recovery is
//! recovered every time.

use super::part::{answering_after_reset, Answer};
use super::recover_limit_tests::{present, RECOVERY};
use super::stall::{health, stalled, whole_flush};
use crate::clock::{now_ms, pass};
use crate::protocol::E_DEVICE;
use crate::server::recover::policy::{IN_A_ROW, SERVED_MS};
use crate::server::recover::settle;
use crate::state::Health;

#[test]
fn a_device_that_answers_one_present_per_recovery_is_down_in_bounded_time() {
    let (mut d, region, regs) = stalled();
    let _part = answering_after_reset(&region, &regs, Answer::Spec, RECOVERY + 1);
    pass(SERVED_MS);
    let from = now_ms();
    settle(&mut d);
    let mut cycles = 0;
    while d.health == Health::Live && cycles < IN_A_ROW * 3 {
        assert_eq!(present(&mut d, &whole_flush()), 0, "the device answered the present");
        assert_eq!(present(&mut d, &whole_flush()), E_DEVICE, "the next one stalled");
        cycles += 1;
    }
    assert_eq!((d.health, cycles), (Health::Down, IN_A_ROW), "recovered past the count");
    assert_eq!(health(&d), (1, IN_A_ROW), "no recovery past the count");
    let spent = now_ms() - from;
    assert!(spent < u64::from(IN_A_ROW) * SERVED_MS, "down only after {spent} ms");
}

#[test]
fn a_device_that_serves_through_the_span_after_each_recovery_is_kept_live() {
    let (mut d, region, regs) = stalled();
    let _part = answering_after_reset(&region, &regs, Answer::Spec, RECOVERY + 2);
    settle(&mut d);
    for _ in 0..IN_A_ROW * 2 {
        assert_eq!(present(&mut d, &whole_flush()), 0, "the device answered the present");
        pass(SERVED_MS);
        assert_eq!(present(&mut d, &whole_flush()), 0, "and one SERVED_MS on");
        assert_eq!(present(&mut d, &whole_flush()), E_DEVICE, "the next one stalled");
    }
    assert_eq!(health(&d), (0, 1 + IN_A_ROW * 2), "a device that serves is kept Live");
}
