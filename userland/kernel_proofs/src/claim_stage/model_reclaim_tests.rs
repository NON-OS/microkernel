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

use super::reclaim_world::{claim, release, World};
use super::types::Stage;

/// Whether some order leaves a grant whose claim is gone, or the newer claim
/// standing, made to the end, without the grant it made (L71).
fn leaks(w: &World, drains: bool, bound: bool) -> bool {
    let done = |s: u8| s >= 4;
    if done(w.rel[0].0) && done(w.rel[1].0) && w.new.0 >= 5 {
        // At 5 the claiming thread ran to the end and pushed its grant.
        let starved = matches!(w.claim, Some((7, _, _))) && w.new.0 == 5 && !w.grants.contains(&7);
        return starved || w.grants.iter().any(|&g| w.claim.is_none_or(|(e, _, _)| e != g));
    }
    let next = |f: &dyn Fn(&mut World) -> bool| {
        let mut n = w.clone();
        f(&mut n) && leaks(&n, drains, bound)
    };
    (!done(w.rel[0].0) && next(&|n| release(n, 0, drains, bound)))
        || (!done(w.rel[1].0) && next(&|n| release(n, 1, drains, bound)))
        || (w.new.0 < 5
            && next(&|n| {
                claim(n);
                true
            }))
}

#[test]
fn a_release_bound_to_its_own_epoch_never_takes_a_newer_claim() {
    let start =
        World { claim: Some((1, Stage::Ready, 0)), grants: vec![1], rel: [(0, 0); 2], new: (0, 0) };
    assert!(!leaks(&start, true, true));
    assert!(leaks(&start, true, false), "the old release removed the newer claim, its grant left");
    assert!(leaks(&start, false, true), "the old drain took the standing newer claim's grant");
}
