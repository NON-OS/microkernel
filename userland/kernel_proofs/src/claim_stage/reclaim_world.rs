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

use super::stage_rule::{grantable, kept, retired};
use super::types::Stage;

// Two releases of one claim racing a third thread that claims the device again
// and maps it, every order tried. `drains`: each drain takes only the grants
// made at the epoch its retire found (L71), the old code all of them; `bound`:
// each release removes that claim only (L69), the old code whatever it finds.
#[derive(Clone)]
pub(super) struct World {
    pub(super) claim: Option<(u64, Stage, u32)>,
    pub(super) grants: Vec<u64>,
    pub(super) rel: [(u8, u64); 2],
    pub(super) new: (u8, u64),
}

pub(super) fn release(w: &mut World, r: usize, drains: bool, bound: bool) -> bool {
    let (step, mine) = w.rel[r];
    match (step, w.claim) {
        (0, Some((e, s, n))) if retired(s).is_ok() => {
            (w.claim, w.rel[r].1) = (Some((e, Stage::Leaving, n)), e)
        }
        (0, _) => {
            return {
                w.rel[r].0 = 9;
                true
            }
        }
        (1, Some((e, _, n))) if e == mine && n > 0 => return false,
        (2, _) if drains => w.grants.retain(|&g| g != mine),
        (2, _) => w.grants.clear(),
        (3, Some((e, s, _))) if (e == mine || !bound) && kept(s) => w.claim = None,
        _ => {}
    }
    w.rel[r].0 += 1;
    true
}

pub(super) fn claim(w: &mut World) {
    match (w.new.0, w.claim) {
        (0, None) => (w.claim, w.new.1) = (Some((7, Stage::Making, 0)), 7),
        (0, Some(_)) => w.new.0 = 9,
        (1, Some((7, Stage::Making, n))) => w.claim = Some((7, Stage::Ready, n)),
        (2, Some((7, s, n))) if grantable(s) => w.claim = Some((7, s, n + 1)),
        (2, _) => w.new.0 = 9,
        (3, Some((7, s, _))) if kept(s) => w.grants.push(7),
        (4, Some((7, s, n))) => w.claim = Some((7, s, n - 1)),
        _ => {}
    }
    w.new.0 += 1;
}
