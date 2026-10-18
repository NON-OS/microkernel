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

//! One fill against a device that answers: the doorbell, the bytes handed
//! on, and a wait that ends by the clock, never by a count of polls
//!. The wait used to be 100,000 yields, whose length was the
//! scheduler's load.

use nonos_devmodel::FakeBar;

use super::drive::answered;
use super::model::{device, regs, view};
use super::ring::{Ring, SIZE};
use crate::clock::expired_after;
use crate::constants::LEG_QUEUE_NOTIFY;
use crate::fill::fill;
use crate::queue::Fault;
use crate::transport::{Modern, Transport};

#[test]
fn a_legacy_fill_rings_queue_zero_and_hands_on_the_answer() {
    let (bar, ring) = (device(SIZE), Ring::new());
    bar.present16(LEG_QUEUE_NOTIFY, 0xFFFF);
    let mut q = ring.queue();
    let out = answered(Transport::Legacy(regs(&bar)), &mut q, &ring, 0x77);
    assert_eq!(out, Ok([0x77; 64]));
    assert_eq!(bar.wrote16(LEG_QUEUE_NOTIFY), 0, "the doorbell rang for queue zero");
}

#[test]
fn a_modern_fill_rings_the_queues_own_doorbell() {
    let (common, notify, ring) = (FakeBar::new(0x40), FakeBar::new(0x40), Ring::new());
    notify.present16(12, 0xFFFF);
    let modern = Modern { common: view(&common), notify: view(&notify), doorbell: 12 };
    let mut q = ring.queue();
    assert_eq!(answered(Transport::Modern(modern), &mut q, &ring, 0x66), Ok([0x66; 64]));
    assert_eq!(notify.wrote16(12), 0, "queue zero's doorbell, where the device put it");
}

#[test]
fn the_wait_ends_when_the_deadline_passes_not_after_a_count() {
    for ms in [1u64, 2, 500, 5_000] {
        let (bar, ring) = (device(SIZE), Ring::new());
        let mut q = ring.queue();
        let (mut asked, mut clock) = (0u64, expired_after(ms));
        let tick = || {
            asked += 1;
            clock()
        };
        let got = fill(Transport::Legacy(regs(&bar)), &mut q, &mut [0u8; 8], tick);
        assert_eq!(got, Err(Fault::TimedOut), "{ms}");
        assert_eq!(asked, ms, "the wait asked the clock until it said the time was up");
    }
}
