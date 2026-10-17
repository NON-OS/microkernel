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
//! The blank gone wrong at every command, for a dead owner and for a
//! silent one: the host dead from there, for a while or for good, or the
//! driver killed there. Whatever landed, nothing of the owner's stays where
//! a landed command took it off; the primary is zero once the driver got as
//! far as sending it; and a host back again has the next wakes finish it.

use super::draw::{draw, primary_px};
use super::judge::{clean, judge};
use super::kit::{run, wrongs, Wrong};
use super::rig::rig;
use crate::death::watch::watch;

fn blank_steps() -> u32 {
    let r = rig();
    draw(&r);
    let from = r.gpu.borrow().steps();
    assert!(watch(&r.d, 100, false));
    let to = r.gpu.borrow().steps();
    to - from
}

fn sweep(now: u64, alive: bool) {
    let n = blank_steps();
    assert!(n >= 6, "{n} commands in a blank");
    for w in wrongs(n) {
        let r = rig();
        let own = draw(&r);
        let from = r.gpu.borrow().steps();
        let done = run(&r, w, || watch(&r.d, now, alive));
        let tag = format!("{w:?} of {n}");
        judge(&r, from, own, done.is_some(), &tag);
        let sent_zero = r.gpu.borrow().log[from as usize..].iter().any(|c| c.kind == 0x0105);
        if done.is_some() || sent_zero {
            assert!(primary_px(&r).iter().all(|&p| p == 0), "{tag}: the primary zeroed");
        }
        let Wrong::Dead(k, d) = w else { continue };
        assert_eq!(done, Some(true), "{tag}: a watch that ran");
        assert_eq!(r.d.owner.quiet.get(), k > n, "{tag}: whole only if nothing was lost");
        if d > 0 || k > n {
            // The dead window may reach into the next wake: at most d more.
            let tries = (1..=d as u64 + 1).take_while(|&t| watch(&r.d, now + 500 * t, alive));
            assert!(tries.count() as u32 <= d, "{tag}: finished within the window");
            clean(&r, own, &tag);
        }
    }
}

#[test]
fn a_dead_owner_s_blank_wrong_at_any_command_leaves_none_of_its_frame_shown() {
    sweep(100, false);
}

#[test]
fn a_silent_owner_s_blank_wrong_at_any_command_leaves_none_of_its_frame_shown() {
    sweep(3_000, true);
}
