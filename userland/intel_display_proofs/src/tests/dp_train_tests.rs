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

//! Link training against the sink model.

use crate::dp::caps::Caps;
use crate::dp::lane::Drive;
use crate::dp::train::{train, Pattern, TrainError, Trained};
use crate::model::sink::{sink, Sink};
use crate::model::sink_aux::AuxEnd;
use crate::model::sink_link::LinkEnd;

pub(super) fn go(s: &Sink, max: (u32, u32)) -> Result<Trained, TrainError> {
    let caps = Caps::read(&mut AuxEnd(s.clone())).unwrap().unwrap();
    train(&mut AuxEnd(s.clone()), &mut LinkEnd(s.clone()), &caps, max)
}

#[test]
fn the_fastest_widest_link_trains_with_the_drive_the_sink_asks() {
    let s = sink(vec![]);
    let t = go(&s, (8100, 4)).unwrap();
    assert_eq!((t.mbps, t.lanes, t.drive), (8100, 4, Drive { swing: 1, pre: 1 }));
    let st = s.borrow();
    assert_eq!(&st.dpcd[0x100..0x102], &[0x1E, 0x84], "HBR3, four lanes, enhanced");
    assert_eq!((st.dpcd[0x102], st.pattern), (0, Pattern::Off), "training off at the end");
    assert_eq!(st.dpcd[0x103], Drive { swing: 1, pre: 1 }.lane_set());
    assert!(st.waited_us >= 100 + 400);
}

#[test]
fn a_sink_that_locks_only_slower_gets_the_slower_rate() {
    let s = sink(vec![]);
    s.borrow_mut().lock_mbps = 2700;
    let t = go(&s, (8100, 4)).unwrap();
    assert_eq!((t.mbps, t.lanes), (2700, 4));
    assert_eq!(s.borrow().stops, 2, "8.1 and 5.4 taken down");
}

#[test]
fn a_sink_that_locks_only_narrower_steps_down_the_lanes() {
    let s = sink(vec![]);
    s.borrow_mut().lock_lanes = 2;
    assert_eq!(go(&s, (8100, 4)).map(|t| (t.mbps, t.lanes)), Ok((8100, 2)));
}

#[test]
fn the_source_limits_cap_the_attempt() {
    let s = sink(vec![]);
    assert_eq!(go(&s, (5400, 1)).map(|t| (t.mbps, t.lanes)), Ok((5400, 1)));
}
