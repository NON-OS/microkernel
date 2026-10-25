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

//! The takeover's step orders undone one at a time (L79), each a knob on
//! the model: over starts where the model as built breaks nothing, the knob
//! breaks the invariant named for it in some order of steps.

use super::knob::Knob;
use super::model_tests::built;
use super::search::sweep;
use super::start_sets::some;
use super::starts::Cfg;
use super::world_check::{DARK, KERNEL_DRAWS, SPURIOUS, TWO_WRITERS, WIPE_LEFT};
use super::world_names::names;

/// What `k` breaks over the starts `keep` keeps, once the build is shown to
/// break nothing there; at least `floor` worlds searched under the knob.
pub(super) fn undone(k: Knob, keep: impl Fn(&Cfg) -> bool, floor: usize) -> u16 {
    let clean = built().iter().filter(|(c, _)| keep(c)).all(|(_, f)| f.broken == 0);
    assert!(clean, "{k:?}'s starts as built");
    let f = sweep(&some(keep), k);
    assert!(f.worlds >= floor, "{k:?}: {f:?}");
    f.broken
}

pub(super) fn breaks(k: Knob, keep: impl Fn(&Cfg) -> bool, floor: usize, wanted: u16) {
    let broken = undone(k, keep, floor);
    assert_eq!(broken & wanted, wanted, "{k:?}: {:?}", names(broken));
}

#[test]
fn gone_latched_at_the_stop_leaves_the_console_dark_after_a_failed_admit() {
    breaks(Knob::GoneEarly, |c| !c.g2 && c.doomed && !c.again, 20_000, DARK);
}

#[test]
fn a_release_under_way_read_as_done_lets_the_gpu_in_beside_bootfb() {
    breaks(Knob::LeavingDone, |c| !c.g2 && !c.doomed, 100_000, TWO_WRITERS);
}

#[test]
fn the_takeover_after_the_admit_lets_the_gpu_in_under_a_kernel_draw() {
    breaks(Knob::TakeAfterAdmit, |c| !c.g2 && !c.doomed && !c.again, 20_000, KERNEL_DRAWS);
    // Told to ask again before the takeover ever looked at bootfb's claim.
    breaks(Knob::TakeAfterAdmit, |c| !c.g2 && !c.doomed && !c.again, 20_000, SPURIOUS);
}

#[test]
fn the_wipe_record_before_the_scanout_check_outlives_a_refused_admit() {
    breaks(Knob::ScanoutAfterWipe, |c| !c.g2 && !c.doomed && !c.again, 20_000, WIPE_LEFT);
}
