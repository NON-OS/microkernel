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
//! Ids the driver did not make in this life are believed of no one: one a
//! client names that the table holds (the primary's, the blank's, one in
//! use) is refused before the host hears of it, a fresh id skips every one
//! held, and a restarted driver refuses an id a dead one handed out.

use super::ask::{ask, words, OWNER};
use super::draw::{draw, SIDE};
use super::rig::rig;
use crate::protocol::{E_BUSY, E_INVAL, OP_CREATE_RESOURCE, OP_FLUSH, OP_SET_SCANOUT};

fn create(r: &super::rig::Rig, id: u32) -> (i32, u32) {
    let (status, body) = ask(r, OWNER, OP_CREATE_RESOURCE, &words(&[id, 1, SIDE, SIDE]), 0);
    (status, body.get(..4).map_or(0, |b| u32::from_le_bytes(b.try_into().unwrap())))
}

#[test]
fn an_id_the_table_holds_is_refused_before_the_host_hears_of_it() {
    let r = rig();
    let own = draw(&r);
    let held = [r.d.primary.unwrap().resource_id, r.d.blank.unwrap().resource_id, own];
    let before = r.gpu.borrow().steps();
    for id in held {
        assert_eq!(create(&r, id).0, E_BUSY, "id {id}");
    }
    assert_eq!(r.gpu.borrow().steps(), before, "nothing sent");
}

#[test]
fn a_fresh_id_skips_every_one_held() {
    let r = rig();
    let own = draw(&r);
    assert_eq!(create(&r, own + 2), (0, own + 2));
    assert_eq!(create(&r, 0), (0, own + 1));
    assert_eq!(create(&r, 0), (0, own + 3), "past the one a client chose");
}

#[test]
fn a_restarted_driver_refuses_an_id_the_dead_one_handed_out() {
    let dead = rig();
    draw(&dead);
    create(&dead, 0);
    let (_, old) = create(&dead, 0);
    drop(dead);
    let r = rig();
    assert!(draw(&r) < old, "this life made fewer");
    let p = r.d.primary.unwrap();
    let before = r.gpu.borrow().steps();
    let rect = words(&[old, 0, 0, SIDE, SIDE]);
    assert_eq!(ask(&r, OWNER, OP_FLUSH, &rect, 0).0, E_INVAL);
    let shown = words(&[0, old, 0, 0, p.width, p.height]);
    assert_eq!(ask(&r, OWNER, OP_SET_SCANOUT, &shown, 0).0, E_INVAL);
    assert_eq!(r.gpu.borrow().steps(), before, "nothing sent");
}
