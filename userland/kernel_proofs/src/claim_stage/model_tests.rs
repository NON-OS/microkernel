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

use super::stage_rule::{grantable, retired};
use super::types::Stage;

// A grant path's steps (check, record, done) and a release's (retire, wait,
// drain), run in every order a scheduler could pick.
#[derive(Clone, Copy)]
struct World {
    stage: Stage,
    inside: u32,
    grants: u32,
    map: u8,
    rel: u8,
    admitted: bool,
}

fn step_map(w: &mut World) {
    match w.map {
        0 if grantable(w.stage) => (w.inside, w.admitted) = (w.inside + 1, true),
        0 => w.map = 2,
        1 if w.admitted => w.grants += 1,
        2 if w.admitted => w.inside -= 1,
        _ => {}
    }
    w.map += 1;
}

/// False while the release is waiting on a path still inside.
fn step_rel(w: &mut World, waits: bool) -> bool {
    match w.rel {
        0 => w.stage = retired(w.stage).unwrap_or(w.stage),
        1 if waits && w.inside > 0 => return false,
        2 => w.grants = 0,
        _ => {}
    }
    w.rel += 1;
    true
}

/// Whether some order leaves a grant after the drain.
fn leaks(w: World, waits: bool) -> bool {
    if w.map >= 3 && w.rel >= 3 {
        return w.grants > 0;
    }
    let mut a = w;
    let by_map = w.map < 3 && {
        step_map(&mut a);
        leaks(a, waits)
    };
    let mut b = w;
    by_map || (w.rel < 3 && step_rel(&mut b, waits) && leaks(b, waits))
}

#[test]
fn no_grant_outlives_a_release_that_waits_out_the_paths_inside() {
    let start =
        World { stage: Stage::Ready, inside: 0, grants: 0, map: 0, rel: 0, admitted: false };
    assert!(!leaks(start, true));
    assert!(leaks(start, false), "draining without the wait leaves the late grant");
}
