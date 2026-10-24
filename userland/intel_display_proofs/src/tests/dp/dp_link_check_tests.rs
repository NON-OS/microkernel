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

//! Link maintenance: a trained link that still holds, one that a sink
//! lost and is trained again, and an AUX channel that does not answer.

use super::dp_train_tests::go;
use crate::dp::link_check::link_lost;
use crate::model::sink::sink;
use crate::model::sink_aux::AuxEnd;

#[test]
fn a_trained_link_holds_until_the_sink_loses_it_then_trains_again() {
    let s = sink(vec![]);
    let t = go(&s, (8100, 4)).unwrap();
    assert_eq!(link_lost(&mut AuxEnd(s.clone()), t.lanes), Ok(false));
    s.borrow_mut().lock_mbps = 2700;
    assert_eq!(link_lost(&mut AuxEnd(s.clone()), t.lanes), Ok(true), "the cable gave way");
    let again = go(&s, (8100, 4)).unwrap();
    assert_eq!((again.mbps, again.lanes), (2700, 4));
    assert_eq!(link_lost(&mut AuxEnd(s.clone()), again.lanes), Ok(false));
}

#[test]
fn a_dropped_transmitter_or_a_silent_channel_is_not_a_held_link() {
    let s = sink(vec![]);
    let t = go(&s, (8100, 4)).unwrap();
    s.borrow_mut().link = None;
    assert_eq!(link_lost(&mut AuxEnd(s.clone()), t.lanes), Ok(true));
    s.borrow_mut().nack = true;
    assert!(link_lost(&mut AuxEnd(s.clone()), t.lanes).is_err());
}
