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

//! One step of each path in the exit world (L75): false when it cannot move.

use super::exit_world::{code, Entry, World};
use super::stage_rule::{exited, grantable, kept, retired};
use super::types::Stage;

pub(super) fn step(w: &mut World, p: usize, new: bool) -> bool {
    let s = w.ours();
    match (p, w.steps[p]) {
        (0, 0) if s.is_some_and(grantable) => w.edit(new, |e, _| e.3 += 1),
        (0, 0) => w.steps[0] = 8,
        (0, 1) => w.write(),
        (0, 2) if s.is_some_and(kept) => w.grant = true,
        (0, 2) => w.write(),
        (0, 3) => w.edit(new, |e, _| e.3 -= 1),
        // Nothing of the old claim's left to tear down: the exit does nothing.
        (1, 0) if !s.is_some_and(|s| exited(s).1) => w.steps[1] = 8,
        (1, 0) if new => w.edit(new, |e, s| (e.1, e.2) = (code(exited(s).0), e.2 + 1)),
        (1, 0) => w.entry = None,
        (1, 1) if new || w.entry.is_none() => w.stop(),
        (1, 2) => w.free(),
        (1, 3) => w.edit(new, |e, _| e.2 = e.2.saturating_sub(new as u32)),
        (2, 0) if s.is_some_and(|s| retired(s).is_ok()) => {
            w.edit(new, |e, _| (e.1, e.2) = (code(Stage::Leaving), e.2 + new as u32))
        }
        (2, 0) => w.steps[2] = 8,
        (2, 1) if w.entry.is_some_and(|e| e.0 == 1 && e.3 > 0) => return false,
        (2, 2) if s.is_some() => w.stop(),
        (2, 3) => w.free(),
        (2, 4) if new && s.is_some() => {
            w.stop();
            w.edit(new, |e, _| e.2 -= 1)
        }
        (2, 4) if s.is_some() => (w.entry, w.bus_on) = (None, false),
        (3, 0) if w.entry.is_some() => return false,
        (3, 0) => (w.entry, w.bus_on) = (Some(Entry(7, code(Stage::Ready), 0, 0)), false),
        (3, 1) => w.bus_on = true,
        // Only a wait that yields parks a killed thread: it never comes back;
        // one that spins runs on to its release, the kill landing as nothing.
        (4, 0) if w.yields && w.steps[2] == 1 && w.entry.is_some_and(|e| e.0 == 1 && e.3 > 0) => {
            w.steps[2] = 9
        }
        _ => {}
    }
    w.steps[p] += 1;
    true
}
