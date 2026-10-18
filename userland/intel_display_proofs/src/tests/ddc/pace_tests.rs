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

//! DDC/CI's timing: a reply read no sooner than 40 ms after its request,
//! no two messages closer than 50 ms, and a reading answered from memory
//! for a second.

use super::caps_rig::known;
use super::fake_bus::FakeBus;
use crate::ddc::reply::Reading;
use crate::ddc::transact::read;
use crate::ddc::transact_write::write;
use crate::model::Ticks;

const MONITOR: [(u8, u16, u16); 2] = [(0x10, 70, 100), (0x12, 50, 100)];

#[test]
fn the_reply_waits_40_ms_and_the_next_message_50() {
    let (t, mut p) = (Ticks::new(), known());
    let mut b = FakeBus::new(&t, &MONITOR);
    assert_eq!(read(&mut b, &t, &mut p, 0x10), Ok(Reading { present: 70, max: 100 }));
    assert_eq!(read(&mut b, &t, &mut p, 0x12), Ok(Reading { present: 50, max: 100 }));
    let l = &b.log;
    assert_eq!(l.iter().map(|e| e.0).collect::<Vec<_>>(), [true, false, true, false]);
    assert!(l[1].1 - l[0].1 >= 40_000, "reply read {} us after its request", l[1].1 - l[0].1);
    assert!(l[2].1 - l[1].1 >= 50_000, "next request {} us after the reply", l[2].1 - l[1].1);
}

#[test]
fn a_reading_is_kept_a_second_and_a_write_forgets_it() {
    let (t, mut p) = (Ticks::new(), known());
    let mut b = FakeBus::new(&t, &MONITOR);
    for _ in 0..50 {
        assert_eq!(read(&mut b, &t, &mut p, 0x10).map(|r| r.present), Ok(70));
    }
    assert_eq!(b.sends().len(), 1, "fifty asks, one request");
    assert_eq!(write(&mut b, &t, &mut p, 0x10, 30), Ok(()));
    assert_eq!(b.monitor.value(0x10), Some(30));
    assert_eq!(
        b.monitor.got.last().unwrap(),
        &[0x51, 0x84, 0x03, 0x10, 0x00, 0x1E, 0x6E ^ 0x51 ^ 0x84 ^ 0x03 ^ 0x10 ^ 0x1E]
    );
    assert_eq!(read(&mut b, &t, &mut p, 0x10).map(|r| r.present), Ok(30), "asked again");
    let s = b.sends();
    assert!(s.windows(2).all(|w| w[1] - w[0] >= 50_000), "spaced: {s:?}");
    t.now.set(t.now.get() + 1_000_000);
    assert_eq!(read(&mut b, &t, &mut p, 0x10).map(|r| r.present), Ok(30));
    assert_eq!(b.sends().len(), s.len() + 1, "a second on, asked afresh");
}
