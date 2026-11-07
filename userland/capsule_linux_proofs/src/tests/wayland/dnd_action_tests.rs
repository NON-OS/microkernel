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

//! The action a drop is to be, as wl_data_device 3 negotiates it, and the
//! drag's state as the server keeps it.

use crate::wayland_dnd::action::{chosen, one_or_none, ASK, COPY, MOVE, NONE};
use crate::wayland_dnd::state::{Dnd, Drag};

#[test]
fn the_preferred_action_wins_when_the_source_offers_it() {
    assert_eq!(chosen(COPY | MOVE, COPY | MOVE | ASK, MOVE), MOVE);
    assert_eq!(chosen(COPY | MOVE | ASK, ASK, ASK), ASK);
}

#[test]
fn else_the_first_both_share_copy_before_move_before_ask() {
    assert_eq!(chosen(COPY | MOVE, COPY | MOVE, NONE), COPY);
    assert_eq!(chosen(MOVE | ASK, COPY | MOVE | ASK, COPY), MOVE);
    assert_eq!(chosen(ASK, MOVE | ASK, MOVE), ASK);
}

#[test]
fn nothing_shared_is_none() {
    assert_eq!(chosen(COPY, MOVE, MOVE), NONE);
    assert_eq!(chosen(NONE, COPY | MOVE, COPY), NONE);
    // Bits past ask are no action.
    assert_eq!(chosen(8, 8, 8), NONE);
}

#[test]
fn the_preferred_action_is_one_or_none() {
    assert!(one_or_none(NONE) && one_or_none(MOVE) && one_or_none(ASK));
    assert!(!one_or_none(COPY | MOVE));
    assert!(!one_or_none(8));
}

fn drag(dropped: bool) -> Drag {
    Drag {
        source: Some(7),
        over: Some(3),
        offers: vec![(5, 0xff00_0000)],
        accepted: false,
        dest: (0, 0),
        action: NONE,
        dropped,
    }
}

#[test]
fn a_source_with_no_actions_set_offers_every_one() {
    let mut d = Dnd::default();
    assert_eq!(d.offered(Some(7)), COPY | MOVE | ASK);
    d.actions.push((7, COPY));
    assert_eq!(d.offered(Some(7)), COPY);
    assert_eq!(d.offered(None), COPY | MOVE | ASK);
}

#[test]
fn a_dropped_drag_no_longer_holds_the_pointer() {
    let mut d = Dnd { drag: Some(drag(false)), ..Default::default() };
    assert!(d.dragging() && d.offer_of(0xff00_0000) && !d.offer_of(1));
    d.drag = Some(drag(true));
    assert!(!d.dragging());
    assert!(d.offer_of(0xff00_0000), "its offer still finishes it");
}
