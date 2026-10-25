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

//! The model takes the interleavings the kernel's locks allow, not only the
//! tidy ones: bootfb's claim can be released, and HELD cleared, while the
//! hold's wait still runs outside the claims' lock, and draws go on in that
//! window. And GONE is stored only as a takeover's claim is made ready, so a
//! refused takeover never darkens the console.

use std::collections::HashSet;

use super::actors::{Hold, Taker, World};
use super::gate::Gate;
use super::moves::{apply, moves};

type Want<'a> = &'a dyn Fn(&World) -> bool;

/// Whether some interleaving from `w` reaches a world `want` holds of.
fn reaches(g: &Gate, w: World, want: Want, seen: &mut HashSet<World>) -> bool {
    if want(&w) {
        return true;
    }
    if !seen.insert(w) {
        return false;
    }
    let next = moves(&w).into_iter().filter_map(|m| apply(g, &w, m).map(|n| n.0));
    let next: Vec<World> = next.collect();
    next.into_iter().any(|n| reaches(g, n, want, seen))
}

fn from(takers: usize, want: Want) -> bool {
    let g = Gate::KERNEL;
    reaches(&g, World::new(&g, 2, takers, true), want, &mut HashSet::new())
}

#[test]
fn a_release_clears_held_while_the_holds_wait_runs_and_a_draw_goes_on() {
    let drawing = |w: &World| w.drawers.iter().any(|d| d.drawing());
    assert!(from(1, &|w| w.hold == Hold::Waiting && drawing(w)));
    assert!(from(1, &|w| w.hold == Hold::Waiting && w.takers[0] == Taker::Latched));
}

#[test]
fn gone_is_stored_only_as_a_takeovers_claim_is_made_ready() {
    let made = |t: &Taker| matches!(t, Taker::Latched | Taker::Ended { ready: true });
    assert!(!from(2, &|w| w.flags.gone && !w.takers.iter().any(made)));
    assert!(from(2, &|w| w.flags.gone), "no claim was ever made ready");
}
