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

//! The takeover model as built (L79): in every order of every start each
//! invariant holds, and the ends that matter are reached, counted against
//! floors so a search that reached nothing could not pass.

use std::sync::OnceLock;

use super::events::{ASKED_AGAIN, ORPHANED};
use super::found::Found;
use super::knob::Knob;
use super::search::sweep;
use super::start_sets::every;
use super::starts::Cfg;
use super::world_names::names;

/// Every start's search as built, run once for every test that asks.
pub(super) fn built() -> &'static [(Cfg, Found)] {
    static ALL: OnceLock<Vec<(Cfg, Found)>> = OnceLock::new();
    ALL.get_or_init(|| every().into_iter().map(|c| (c, sweep(&[c], Knob::Built))).collect())
}

fn total(keep: impl Fn(&Cfg) -> bool) -> Found {
    let mut t = Found::default();
    for (_, f) in built().iter().filter(|(c, _)| keep(c)) {
        (t.worlds, t.ends, t.events) = (t.worlds + f.worlds, t.ends + f.ends, t.events | f.events);
        (t.ready, t.taken, t.failed) = (t.ready + f.ready, t.taken + f.taken, t.failed + f.failed);
    }
    t
}

#[test]
fn every_order_of_every_start_keeps_every_invariant() {
    for (c, f) in built() {
        assert_eq!(f.broken, 0, "{c:?}: {:?}", names(f.broken));
    }
    let all = total(|_| true);
    assert!(all.worlds >= 2_500_000 && all.ends >= 3_000, "{all:?}");
}

#[test]
fn the_gpu_is_made_ready_and_bootfb_taken_back_in_every_kind_of_start() {
    let all = total(|_| true);
    assert!(all.ready >= 1_300 && all.taken >= 800, "{all:?}");
    assert_eq!(all.events, (1 << 15) - 1, "some step's case never reached: {all:?}");
    for exit in [false, true] {
        let some = total(|c| c.exit == exit && !c.doomed);
        assert!(some.ready >= 200 && some.taken >= 100, "exit {exit}: {some:?}");
    }
}

#[test]
fn a_claim_refused_at_its_admit_leaves_the_console_as_it_was() {
    // G1's holder exits in its claim's middle: refused in every end.
    let doomed = total(|c| c.doomed);
    assert!(doomed.ready == 0 && doomed.failed == doomed.ends, "{doomed:?}");
    assert!(doomed.ends >= 1_000 && doomed.events & ORPHANED != 0, "{doomed:?}");
    // Else refused only when bootfb's claim was made ready meanwhile.
    let live = total(|c| !c.doomed);
    assert!(live.failed >= 250 && live.events & ASKED_AGAIN != 0, "{live:?}");
}
