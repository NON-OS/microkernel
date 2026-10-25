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

//! The lid through random sessions: closes and opens, damage anywhere, the
//! band's capturers and trusted mark changing, presents refused. Whenever
//! the lid is dark the screen is all black, and every open frame that went
//! out leaves the screen drawn whole for what it shows now.

use super::lid_rig::{black, whole, Rig, Shows, H, W};
use crate::damage::Rect;
use crate::frame_pacer::indicator_band::band;
use crate::state::indicator::KIND_MICROPHONE;
use crate::state::lid_rule::Lid;
use crate::tests::scene::frame_util::Rng;

#[test]
fn through_any_session_dark_is_black_and_open_is_whole() {
    for seed in 1..=48 {
        let (mut rng, mut s) = (Rng::new(seed), Shows::default());
        let mut rig = Rig::new(s);
        for i in 0..200 {
            match rng.below(7) {
                0 => rig.shell(rng.below(2) == 0),
                1 => drop(s.ind.apply(KIND_MICROPHONE, rng.below(2) == 0, 1 + rng.below(3))),
                2 => s.trusted = !s.trusted,
                3 => rig.refuse = rng.below(2),
                _ => {
                    let (x, y) = (rng.below(W), rng.below(H));
                    let r =
                        Rect { x, y, width: 1 + rng.below(W - x), height: 1 + rng.below(H - y) };
                    rig.acc.accumulate(r);
                }
            }
            // Every change the band shows damages it, as the band rule does.
            rig.acc.accumulate(band(W, H));
            let before = rig.presents;
            rig.frame(s);
            match rig.lid {
                Lid::Dark => assert!(rig.screen == black(), "seed {seed}, step {i}"),
                Lid::Open if rig.refuse == 0 && rig.presents > before => {
                    assert!(rig.screen == whole(Lid::Open, s), "seed {seed}, step {i}")
                }
                _ => {}
            }
            if rig.lid == Lid::Dark && before == rig.presents {
                assert!(rig.screen == black());
            }
        }
    }
}
