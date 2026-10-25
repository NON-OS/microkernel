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

//! A drawer's loads in `stopped()` and its guard's drop, step by step, and a
//! draw started alone.

use super::drawer::{ended, fresh, step};
use super::events::Event;
use super::gate::Gate;
use super::world::{At, Drawer, Flags};

/// Load `n` of a check; `again` is the one made once counted. A load that
/// finds a stop ends the check: the draw returns, or backs out once counted.
pub fn load(g: &Gate, f: &Flags, d: &mut Drawer, n: u8, again: bool) -> Event {
    let loads = if again { g.again() } else { g.first() };
    let flag = loads[n as usize];
    let stop = f.stop(flag);
    let last = n as usize + 1 == loads.len();
    match (stop, again, last) {
        (true, true, _) if g.backs_out => d.at = At::BackOut,
        (true, _, _) => ended(g, d),
        (false, false, true) => d.at = At::Count,
        (false, true, true) => d.at = At::Write,
        (false, false, false) => d.at = At::First(n + 1),
        (false, true, false) => d.at = At::Again(n + 1),
    }
    Event::Load { flag, again, stop }
}

/// Step `n` of the guard's drop, in the gate's order: the fence drains the
/// draw's stores, the decrement takes it out of DRAWS. That the decrement
/// alone drains no write-combined store is assumed, recalled, not verified.
pub fn finish(g: &Gate, f: &mut Flags, d: &mut Drawer, n: u8) -> Event {
    let fence = (n == 0) == g.fence_first;
    let e = if fence {
        d.pending = false;
        Event::Fence
    } else {
        f.draws -= 1;
        Event::Sub
    };
    if n == 0 {
        d.at = At::End(1);
    } else {
        ended(g, d);
    }
    e
}

/// Whether a draw started alone from `f`, nothing else stepping, writes.
pub fn probe(g: &Gate, f: &Flags) -> bool {
    let (mut f, mut d) = (*f, fresh(g, 0));
    while let Some(e) = step(g, &mut f, &mut d) {
        if e == Event::Write {
            return true;
        }
    }
    false
}
