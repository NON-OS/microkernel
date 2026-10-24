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

//! The capabilities read on the model bus: the whole string in fragments
//! at the offsets it reached, no step that waits, no control asked before
//! the string is in, and a display that does not answer, answers wrong
//! three times, or never ends refused for good.

use super::fake_bus::FakeBus;
use crate::ddc::error::DdcError;
use crate::ddc::pace::Pacer;
use crate::ddc::transact::read;
use crate::engine::access::Clock;
use crate::model::Ticks;

const MONITOR: [(u8, u16, u16); 2] = [(0x10, 70, 100), (0x60, 0x0F, 0x12)];

/// Poll steps 60 ms apart, as the serve loop's polls come; the steps taken.
pub(super) fn polls(b: &mut FakeBus, t: &Ticks, p: &mut Pacer, max: usize) -> usize {
    for i in 0..max {
        let before = t.now.get();
        p.fetch_caps(b, t.now_us().unwrap());
        assert!(t.now.get() - before < 1_000, "a step waited");
        if p.caps.settled() {
            return i + 1;
        }
        t.now.set(t.now.get() + 60_000);
    }
    max
}

#[test]
fn the_string_comes_in_whole_and_only_then_is_a_control_asked() {
    let (t, mut p) = (Ticks::new(), Pacer::new());
    let mut b = FakeBus::new(&t, &MONITOR);
    assert_eq!(read(&mut b, &t, &mut p, 0x10), Err(DdcError::NoCapabilities));
    assert!(b.log.is_empty(), "nothing asked before the string");
    polls(&mut b, &t, &mut p, 100);
    let len = crate::model::mccs_caps::caps_text(&b.monitor).len();
    let offsets: Vec<usize> =
        b.monitor.got.iter().map(|m| (m[3] as usize) << 8 | m[4] as usize).collect();
    let want: Vec<usize> = (0..len).step_by(32).chain([len]).collect();
    assert_eq!(offsets, want, "each fragment from where the last ended, then the end");
    assert!(p.caps.caps().unwrap().has(0x10));
    assert_eq!(read(&mut b, &t, &mut p, 0x10).map(|r| r.present), Ok(70));
}

#[test]
fn a_display_that_does_not_answer_or_answers_wrong_is_refused() {
    let (t, mut p) = (Ticks::new(), Pacer::new());
    let mut b = FakeBus::new(&t, &MONITOR);
    b.absent = true;
    assert_eq!(polls(&mut b, &t, &mut p, 10), 1, "refused at the first request");
    let (t, mut p) = (Ticks::new(), Pacer::new());
    let mut b = FakeBus::new(&t, &MONITOR);
    b.monitor.spoil = Some((3, 0x7F));
    polls(&mut b, &t, &mut p, 50);
    assert_eq!(b.monitor.got.len(), 3, "the same fragment three times");
    assert!(b.monitor.got.iter().all(|m| m[3..5] == [0, 0]));
    assert_eq!(p.caps.caps(), Err(DdcError::NoCapabilities));
    assert_eq!(read(&mut b, &t, &mut p, 0x10), Err(DdcError::NoCapabilities));
}
