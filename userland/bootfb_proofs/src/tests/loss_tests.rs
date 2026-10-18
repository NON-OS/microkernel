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
//! The scanout lost to a GPU driver (G-80, L79) over seeded orders of the
//! compositor's presents, its STATE calls and idle wakes, the kernel taking
//! the scanout before any of them or in the middle of a present. However it
//! falls: nothing faults; the claim is let go of exactly once and the
//! compositor told exactly once; from the first ask after the takeover no
//! frame is drawn, every FLUSH answers ENXIO and every STATE LOST, dark.

use super::loss_play::{play, Ev};

#[test]
fn however_the_takeover_falls_bootfb_ends_answering_lost() {
    let steps = [Ev::Present(0x0011_2233), Ev::Idle, Ev::Look];
    let mut runs = 0;
    for n in 0..3u32.pow(4) {
        let evs: Vec<Ev> = (0..4).map(|d| steps[(n / 3u32.pow(d) % 3) as usize]).collect();
        for at in 0..=evs.len() {
            for take in [Ev::Take, Ev::TakeMid] {
                let mut order = evs.clone();
                order.insert(at, take);
                order.push(Ev::Present(0x0044_5566));
                order.push(Ev::Look);
                play(&order);
                runs += 1;
            }
        }
        play(&evs);
    }
    assert!(runs > 500);
}
