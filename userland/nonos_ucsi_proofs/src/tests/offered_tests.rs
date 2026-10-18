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

//! The offers read from the PPM: only from a Power Delivery charger, four
//! to a command up to seven, ending where the partner's list ends or the
//! PPM refuses, and a change of offers reported as a change of the port.

use super::pdo_tests::fixed;
use super::session_tests::running;
use crate::command::GET_PDOS;

fn offers(n: usize) -> Vec<u32> {
    (1..=n as u32).map(|i| fixed(5_000 * i, 3_000)).collect()
}

#[test]
fn a_chargers_offers_are_read_four_to_a_command() {
    for (have, kept, asks) in [(0, 0, 1), (3, 3, 1), (4, 4, 2), (5, 5, 2), (7, 7, 2), (9, 7, 2)] {
        let (mut m, mut s) = running(1);
        m.offers[0] = offers(have);
        m.plug(1, true, false);
        let before = m.commands.len();
        s.notified(&mut m, 0);
        s.tick(&mut m, 0);
        let asked = m.commands[before..].iter().filter(|c| **c == GET_PDOS).count();
        let o = s.ports[0].offered;
        assert_eq!((usize::from(o.count), asked), (kept, asks), "{have} offered");
        assert_eq!(o.list(), &offers(have)[..kept]);
        assert_eq!(s.take_changed() & 2 != 0, have != 0, "a change of offers is a change");
    }
}

#[test]
fn only_a_power_delivery_charger_is_asked() {
    let (mut m, mut s) = running(1);
    let before = m.commands.len();
    m.offers[0] = offers(2);
    m.plug(1, true, true);
    s.notified(&mut m, 0);
    s.tick(&mut m, 0);
    assert!(!m.commands[before..].contains(&GET_PDOS), "a partner this port powers");
    assert_eq!(s.ports[0].offered.count, 0);
}

#[test]
fn a_refused_read_reads_nothing_and_keeps_the_status() {
    let (mut m, mut s) = running(1);
    (m.offers[0], m.refuse_pdos) = (offers(3), true);
    m.plug(1, true, false);
    s.notified(&mut m, 0);
    s.tick(&mut m, 0);
    assert_eq!(s.ports[0].offered.count, 0);
    assert_eq!(s.ports[0].status.map(|p| p.charger()), Some(true), "the status still read");
}
