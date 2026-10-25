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

//! A verb the controller fetched shows its engine working: the count of
//! verbs never fetched starts again, and the rings are not reset across it.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use super::{link_on, ANSWER, TO_0, TO_1};
use crate::clock::on_poll;
use crate::error::HdaError;
use crate::model::{corb_engine, rings, window};

#[test]
fn a_verb_fetched_between_two_unfetched_keeps_the_rings() {
    let bar = window();
    let (corb, rirb) = rings(ANSWER);
    // An engine that fetches only while the test lets it, stepped from the
    // driver's polls so a change lands between two sends and nowhere else.
    let stuck = Rc::new(Cell::new(true));
    let (b, s) = (Arc::clone(&bar), Rc::clone(&stuck));
    let _model = on_poll(move || {
        if !s.get() {
            corb_engine(&b);
        }
    });
    let mut link = link_on(&bar, corb.base(), rirb.base());
    assert_eq!(link.send(TO_0), Err(HdaError::VerbTimeout), "never fetched");
    stuck.set(false);
    assert_eq!(link.send(TO_1), Err(HdaError::VerbTimeout), "fetched, and codec 1 is silent");
    stuck.set(true);
    assert_eq!(link.send(TO_0), Err(HdaError::VerbTimeout), "never fetched");
    assert_eq!(link.health.unfetched, 1, "a fetched verb did not show the engine working");
    assert_eq!(link.health.resets, 0, "the rings were reset across a fetched verb");
}
