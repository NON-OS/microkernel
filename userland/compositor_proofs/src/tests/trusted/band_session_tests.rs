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

//! The band is never left partly drawn. Frames run as the tick runs them:
//! the band rule, then each drained rectangle drawn with the band clipped to
//! it. Through capturers coming and going, screenshots, the trusted band
//! held and let go, and damage falling anywhere, half over the band
//! included, the band on screen always equals one drawn whole for what it
//! shows now. Without the rule a change under partial damage leaves it
//! mixed, which is why the rule is there.

use super::band_rig::{whole, Rig, Shows, H, W};
use crate::damage::{DamageAccumulator, Rect};
use crate::frame_pacer::indicator_band::band;
use crate::state::indicator::{KIND_CAMERA, KIND_MICROPHONE};
use crate::tests::scene::frame_util::Rng;

fn step(rng: &mut Rng, s: &mut Shows, acc: &mut DamageAccumulator) {
    match rng.below(6) {
        0 => drop(s.ind.apply(KIND_CAMERA, rng.below(2) == 0, 1 + rng.below(3))),
        1 => drop(s.ind.apply(KIND_MICROPHONE, rng.below(2) == 0, 1 + rng.below(3))),
        2 => s.captured = !s.captured,
        3 => s.trusted = !s.trusted,
        _ => {
            let (x, y) = (rng.below(W), rng.below(H));
            let r = Rect { x, y, width: 1 + rng.below(W - x), height: 1 + rng.below(H - y) };
            acc.accumulate(r);
        }
    }
}

#[test]
fn the_band_is_never_left_partly_drawn() {
    for seed in 1..=64 {
        let (mut rng, mut s) = (Rng::new(seed), Shows::default());
        let mut rig = Rig { px: whole(s), drawn: 0, acc: DamageAccumulator::new() };
        for i in 0..150 {
            step(&mut rng, &mut s, &mut rig.acc);
            rig.frame(s, true);
            assert!(rig.px == whole(s), "seed {seed}, step {i}");
        }
    }
}

#[test]
fn without_the_rule_a_change_under_part_damage_is_left_mixed() {
    let mut s = Shows::default();
    let mut rig = Rig { px: whole(s), drawn: 0, acc: DamageAccumulator::new() };
    rig.frame(s, false);
    s.ind.apply(KIND_CAMERA, true, 7).unwrap();
    s.trusted = true;
    // A window's damage over the band's left half only: the camera mark.
    let b = band(W, H);
    rig.acc.accumulate(Rect { x: b.x, y: 0, width: b.width / 2, height: b.height });
    rig.frame(s, false);
    assert!(rig.px != whole(s), "the trusted mark, outside the damage, is stale");
    rig.frame(s, true);
    assert!(rig.px == whole(s), "the rule draws it whole");
}
