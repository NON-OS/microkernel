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

//! The check the loop makes after each interrupt wait: a buffer completion
//! refills a running stream, and marks a one-shot tone played, once.

use nonos_libc::{calls, logged};

use super::poll_irq::poll_irq;
use super::refill::Refill;
use super::serving::{at, playing, Playing};
use crate::audio::PcmQueue;
use crate::constants::{SDSTS_BCIS, SD_STS};
use crate::controller::bdl::PERIOD_BYTES;

/// The engine has finished a buffer.
fn complete(p: &Playing) {
    p.b.sim.bar.present8((p.d.stream_off + SD_STS) as usize, SDSTS_BCIS);
}

#[test]
fn a_completion_on_a_running_stream_refills_the_periods_it_finished() {
    let p = playing();
    let (mut q, mut rf, mut last, mut played) = (PcmQueue::new(), Refill::new(true), 0, false);
    q.push(&vec![9; PERIOD_BYTES as usize]);
    at(&p, PERIOD_BYTES as u32, 0);
    complete(&p);
    let from = calls().len();
    poll_irq(&p.d, &mut last, &mut played, &mut q, &mut rf, true);
    let waited = calls()[from..].iter().any(|c| c.name == "irq_wait" && c.arg == 0);
    assert!(waited, "the wait was not on the polled controller's grant");
    assert_eq!(p.b.pcm.wrote8(0), 9, "the finished period was not refilled");
}

#[test]
fn a_completion_with_no_stream_marks_the_tone_played_once() {
    let p = playing();
    let (mut q, mut rf, mut last, mut played) = (PcmQueue::new(), Refill::new(true), 0, false);
    let from = logged().len();
    complete(&p);
    for _ in 0..3 {
        poll_irq(&p.d, &mut last, &mut played, &mut q, &mut rf, false);
    }
    let done = logged()[from..].iter().filter(|l| l.starts_with("[HDA] play-complete")).count();
    assert!(played && done == 1, "the tone was marked played {done} times");
    assert_eq!(p.b.pcm.wrote8(0), 0, "a stream that was not running was refilled");
}
