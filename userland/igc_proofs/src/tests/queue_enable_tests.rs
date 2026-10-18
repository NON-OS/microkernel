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

//! A queue enable that never reads back is named once the bound has passed,
//! with no slot handed to the part; one that reads back a few looks late is
//! waited for. The part acts on the driver's looks at the clock: the wait
//! takes its deadline between the driver's write and its first read.

use std::{cell::Cell, rc::Rc, sync::Arc, time::Duration};

use nonos_devmodel::FakeBar;
use nonos_libc::{on_clock, OnClock};

use crate::constants::regs::REG_RDT;
use crate::constants::timeouts::QUEUE_ENABLE_MS;
use crate::init::rx_queue::{self, RX_NOT_ENABLED};
use crate::init::tx_queue::{self, TX_NOT_ENABLED};
use crate::regs::Regs;

use super::model::{regs, timed, window};
use super::reset_queue_tests::QUEUES;

type Enable = fn(&Regs) -> Result<(), &'static str>;
const ENABLES: [(Enable, &str); 2] =
    [(rx_queue::enable, RX_NOT_ENABLED), (tx_queue::enable, TX_NOT_ENABLED)];

/// The part holding `queue`'s enable bit clear for `late` looks once the
/// driver has set it, or for good; and the looks it has held it for.
fn held_off(bar: &Arc<FakeBar>, (queue, on): (usize, u32), late: u32) -> (OnClock, Rc<Cell<u32>>) {
    let (b, looks) = (Arc::clone(bar), Rc::new(Cell::new(0u32)));
    let held = Rc::clone(&looks);
    let clock = on_clock(move || {
        let (n, v) = (held.get(), b.wrote32(queue));
        if n > 0 || v & on != 0 {
            b.present32(queue, if n < late { v & !on } else { v | on });
            held.set(n + 1);
        }
    });
    (clock, looks)
}

#[test]
fn a_queue_enable_that_never_reads_back_is_named_on_the_clock() {
    for ((enable, refusal), queue) in ENABLES.into_iter().zip(QUEUES) {
        let bar = window();
        let _clock = held_off(&bar, queue, u32::MAX);
        let (r, took) = timed(|| enable(&regs(&bar)));
        assert_eq!(r, Err(refusal));
        assert!(took >= Duration::from_millis(QUEUE_ENABLE_MS), "gave up after {took:?}");
        assert_eq!(bar.wrote32(REG_RDT), 0, "no receive slot handed to the part");
    }
}

#[test]
fn a_queue_enable_that_reads_back_a_few_looks_late_is_waited_for() {
    for ((enable, _), queue) in ENABLES.into_iter().zip(QUEUES) {
        let bar = window();
        let (_clock, looks) = held_off(&bar, queue, 5);
        assert_eq!(enable(&regs(&bar)), Ok(()));
        assert_eq!(looks.get(), 6, "off for five looks, then on");
    }
}
