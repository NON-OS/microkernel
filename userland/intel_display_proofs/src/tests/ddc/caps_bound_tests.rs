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

//! A capabilities string with no vcp list, or one that never ends, is
//! refused, the second no further into the string than the bound.

use super::caps_fetch_tests::polls;
use super::fake_bus::FakeBus;
use crate::ddc::caps_fetch::CAPS_MAX;
use crate::ddc::pace::Pacer;
use crate::model::Ticks;

const MONITOR: [(u8, u16, u16); 1] = [(0x10, 70, 100)];

#[test]
fn a_string_with_no_list_or_no_end_is_refused() {
    let (t, mut p) = (Ticks::new(), Pacer::new());
    let mut b = FakeBus::new(&t, &MONITOR);
    b.monitor.caps = Some(b"(prot(monitor)type(lcd))".to_vec());
    polls(&mut b, &t, &mut p, 20);
    assert!(p.caps.settled() && p.caps.caps().is_err(), "no vcp list");
    let (t, mut p) = (Ticks::new(), Pacer::new());
    let mut b = FakeBus::new(&t, &MONITOR);
    b.monitor.caps = Some(vec![b' '; CAPS_MAX + 64]);
    polls(&mut b, &t, &mut p, 200);
    assert!(p.caps.settled() && p.caps.caps().is_err(), "past CAPS_MAX");
    assert!(b.monitor.got.len() <= CAPS_MAX / 32 + 2, "read no further than the bound");
}

#[test]
fn a_reply_is_read_no_sooner_than_50_ms_after_its_request() {
    let (t, mut p) = (Ticks::new(), Pacer::new());
    let mut b = FakeBus::new(&t, &MONITOR);
    p.fetch_caps(&mut b, 1_000);
    p.fetch_caps(&mut b, 41_000);
    assert_eq!(b.log.len(), 1, "40 ms on, the reply is not read yet");
    p.fetch_caps(&mut b, 51_000);
    assert_eq!(b.log.iter().map(|l| l.0).collect::<Vec<_>>(), [true, false]);
}
