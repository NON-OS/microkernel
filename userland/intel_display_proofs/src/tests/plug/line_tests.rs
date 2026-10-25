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

//! Hot-plug polling with its two-poll settle.

use crate::engine::hotplug::{live, Poller, DE_PORT_ISR, SDEISR};
use crate::model::Gen9;

#[test]
fn the_lines_read_from_their_two_registers() {
    let mut m = Gen9::new(1);
    m.set(DE_PORT_ISR, 1 << 3);
    m.set(SDEISR, 1 << 22);
    assert_eq!(live(&mut m), [true, false, true, false]);
}

#[test]
fn a_plug_is_reported_once_two_polls_agree() {
    let mut m = Gen9::new(1);
    let mut p = Poller::new(&mut m);
    m.set(SDEISR, 1 << 21);
    assert_eq!(p.poll(&mut m), [None; 4], "one poll is not enough");
    assert_eq!(p.poll(&mut m), [None, Some(true), None, None]);
    assert_eq!(p.poll(&mut m), [None; 4], "reported once");
    m.set(SDEISR, 0);
    p.poll(&mut m);
    assert_eq!(p.poll(&mut m), [None, Some(false), None, None]);
}

#[test]
fn a_bounce_shorter_than_two_polls_is_no_event() {
    let mut m = Gen9::new(1);
    let mut p = Poller::new(&mut m);
    m.set(SDEISR, 1 << 23);
    p.poll(&mut m);
    m.set(SDEISR, 0);
    assert_eq!(p.poll(&mut m), [None; 4]);
    assert_eq!(p.poll(&mut m), [None; 4]);
    assert_eq!(p.known, [false; 4]);
}
