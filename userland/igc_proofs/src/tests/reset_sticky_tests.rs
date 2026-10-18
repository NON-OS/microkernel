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

//! A queue that will not turn off after the reset ends the attempt once the
//! bound has passed; one that turns off a few looks late is waited for. The
//! part acts on the driver's looks at the clock: the wait takes its deadline
//! between the driver's write of 0 and its first read, and turns it on there.

use std::{cell::Cell, rc::Rc, sync::Arc, time::Duration, time::Instant};

use nonos_libc::on_clock;

use crate::constants::regs::REG_RXDCTL;
use crate::constants::{rx_bits::RXDCTL_QUEUE_ENABLE, timeouts::QUEUE_ENABLE_MS};
use crate::init::reset::run;

use super::model::{live, regs, window};
use super::reset_queue_tests::{resets_on, written_off, QUEUES, STILL_ON};

#[test]
fn a_queue_that_will_not_turn_off_ends_the_attempt_on_the_clock() {
    for (queue, enable) in QUEUES {
        let bar = window();
        let _part = live(&bar, resets_on(|_| {}));
        let since = Rc::new(Cell::new(None));
        let (b, on) = (Arc::clone(&bar), Rc::clone(&since));
        let _clock = on_clock(move || {
            if written_off(&b) {
                b.present32(queue, enable);
                on.set(Some(Instant::now()));
            }
        });
        assert_eq!(run(&regs(&bar)), STILL_ON);
        let held = since.get().expect("the queue came back on").elapsed();
        assert!(held >= Duration::from_millis(QUEUE_ENABLE_MS), "gave up after {held:?}");
    }
}

#[test]
fn a_queue_that_turns_off_a_few_looks_late_is_waited_for() {
    let bar = window();
    let _part = live(&bar, resets_on(|_| {}));
    let (b, looks) = (Arc::clone(&bar), Rc::new(Cell::new(0u32)));
    let seen = Rc::clone(&looks);
    let _clock = on_clock(move || {
        let n = seen.get();
        if n > 0 || written_off(&b) {
            b.present32(REG_RXDCTL, if n < 5 { RXDCTL_QUEUE_ENABLE } else { 0 });
            seen.set(n + 1);
        }
    });
    assert_eq!(run(&regs(&bar)), Ok(()));
    assert_eq!(looks.get(), 6, "the queue was on for five looks, then off");
}
