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

//! The control (L79): the same model with the takeover taken out, as the
//! claim path was before it (no stop, no take-back loop, an admit that asks
//! nothing of bootfb's claim), reaches the state the takeover exists to
//! prevent: a GPU driver's claim made ready while bootfb's pages and the
//! kernel's own draws still reach the scanout.

use super::knob::Knob;
use super::search::sweep;
use super::start_sets::some;
use super::world_check::{KERNEL_DRAWS, TWO_WRITERS};
use super::world_names::names;

#[test]
fn without_the_takeover_a_gpu_claim_is_made_ready_beside_bootfb_and_the_console() {
    let starts = some(|c| !c.g2);
    let f = sweep(&starts, Knob::Control);
    let wanted = TWO_WRITERS | KERNEL_DRAWS;
    assert_eq!(f.broken & wanted, wanted, "{:?}", names(f.broken));
    assert!(f.worlds >= 75_000 && f.ready >= 90, "{f:?}");
}
