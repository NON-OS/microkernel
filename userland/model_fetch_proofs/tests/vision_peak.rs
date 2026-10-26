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

//! The rule covers what was measured: need.rs's memory for vl-2b is at least
//! the peak resident memory qwenchat reached answering an image on a NONOS
//! boot, so a machine the rule admits held that run.

#[path = "../../capsule_model_fetch/src/need.rs"]
pub mod need;

use need::memory;

/*
 * vl-2b's two pinned files (file/models/pinned_vl.rs), as default_tier.rs's
 * WEIGHTS sums them; model_fetch_proofs holds that table equal to the pins.
 */
const WEIGHTS: u64 = 1_107_409_952 + 445_053_216;

/*
 * vl-2b shown a 64x64 PNG on a NONOS boot under QEMU (TCG): the metrics line
 * `peak_rss_kb=2365208` on its screen (docs/using/evidence/vision-vl2b/
 * vl-exit.png; the chat's lines are kept off the serial log).
 */
const BOOTED_PEAK: u64 = 2_365_208 * 1024;

#[test]
fn vl_2b_needs_at_least_its_booted_peak() {
    let need = memory("vl-2b", WEIGHTS).unwrap();
    assert!(need >= BOOTED_PEAK, "the rule gives {need}, the boot peaked at {BOOTED_PEAK}");
}
