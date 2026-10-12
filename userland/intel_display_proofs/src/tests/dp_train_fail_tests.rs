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

//! Training that fails, and what it leaves behind.

use super::dp_train_tests::go;
use crate::dp::caps::Caps;
use crate::dp::lane::Drive;
use crate::dp::train::{train, TrainError};
use crate::model::sink::sink;
use crate::model::sink_aux::AuxEnd;
use crate::model::sink_both::Both;

#[test]
fn a_sink_that_never_locks_fails_with_the_port_down() {
    let s = sink(vec![]);
    s.borrow_mut().cr_needs = Drive { swing: 3, pre: 3 };
    assert_eq!(go(&s, (8100, 4)), Err(TrainError::Failed));
    let st = s.borrow();
    assert_eq!((st.link, st.stops), (None, 12), "every rate on every width");
}

#[test]
fn equalisation_that_never_completes_falls_back_too() {
    let s = sink(vec![]);
    s.borrow_mut().eq_needs = Drive { swing: 3, pre: 1 };
    assert_eq!(go(&s, (8100, 4)), Err(TrainError::Failed));
    s.borrow_mut().eq_needs = Drive { swing: 2, pre: 1 };
    assert_eq!(go(&s, (8100, 4)).map(|t| t.drive), Ok(Drive { swing: 2, pre: 1 }));
}

#[test]
fn a_dead_aux_channel_stops_training_at_once() {
    let s = sink(vec![]);
    let caps = Caps::read(&mut AuxEnd(s.clone())).unwrap().unwrap();
    s.borrow_mut().nack = true;
    let r = train(&mut Both(s.clone()), &caps, (8100, 4));
    assert!(matches!(r, Err(TrainError::Aux(_))));
    assert_eq!(s.borrow().stops, 1);
}
