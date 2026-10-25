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

//! A drawer's steps, one atomic operation each: the loads and the count in
//! `drawable()`, the back-out, the draw's stores, and the guard's fence and
//! decrement. A draw stopped before its count returns and tries again.

use super::drawer_steps::{finish, load};
use super::events::Event;
use super::gate::{Flag, Gate};
use super::world::{At, Drawer, Flags};

/// Where a check starts: its first load, or `pass` when it makes none.
pub fn begin(loads: &[Flag], check: At, pass: At) -> At {
    if loads.is_empty() {
        pass
    } else {
        check
    }
}

/// A drawer about to start a draw, with `left` more to try after it.
pub fn fresh(g: &Gate, left: u8) -> Drawer {
    Drawer { at: begin(g.first(), At::First(0), At::Count), left, pending: false }
}

/// The draw is over, drawn or not: the next one, or done.
pub fn ended(g: &Gate, d: &mut Drawer) {
    *d = if d.left > 0 { fresh(g, d.left - 1) } else { Drawer { at: At::Done, ..*d } };
}

/// One step of drawer `d`; None once it is done.
pub fn step(g: &Gate, f: &mut Flags, d: &mut Drawer) -> Option<Event> {
    Some(match d.at {
        At::First(n) => load(g, f, d, n, false),
        At::Again(n) => load(g, f, d, n, true),
        At::Count => {
            f.draws += 1;
            d.at = begin(g.again(), At::Again(0), At::Write);
            Event::Count
        }
        At::BackOut => {
            f.draws -= 1;
            ended(g, d);
            Event::BackOut
        }
        At::Write => {
            (d.pending, d.at) = (true, At::End(0));
            Event::Write
        }
        At::End(n) => finish(g, f, d, n),
        At::Done => return None,
    })
}
