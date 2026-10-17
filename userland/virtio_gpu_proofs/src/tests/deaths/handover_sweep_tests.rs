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
//! A restarted compositor's take gone wrong at every command of the blank
//! it runs first: what landed is judged as for the watch's blank, and the
//! new owner has the primary, zeroed, whatever the host did.

use super::ask::ask;
use super::draw::{draw, primary_px};
use super::handover_tests::NEXT;
use super::judge::judge;
use super::kit::{run, wrongs};
use super::rig::rig;
use crate::protocol::OP_GET_PRIMARY_SURFACE;

#[test]
fn a_take_wrong_at_any_command_still_hands_on_a_zeroed_primary() {
    let n = {
        let r = rig();
        draw(&r);
        let from = r.gpu.borrow().steps();
        ask(&r, NEXT, OP_GET_PRIMARY_SURFACE, &[], 500);
        let to = r.gpu.borrow().steps();
        to - from
    };
    for w in wrongs(n) {
        let r = rig();
        let own = draw(&r);
        let from = r.gpu.borrow().steps();
        let done = run(&r, w, || ask(&r, NEXT, OP_GET_PRIMARY_SURFACE, &[], 500));
        let tag = format!("{w:?} of {n}");
        judge(&r, from, own, done.is_some(), &tag);
        if let Some((status, _)) = done {
            assert_eq!(status, 0, "{tag}: handed on");
            assert_eq!(r.d.owner.pid.get(), NEXT, "{tag}");
            assert!(primary_px(&r).iter().all(|&p| p == 0), "{tag}: zeroed");
        }
    }
}
