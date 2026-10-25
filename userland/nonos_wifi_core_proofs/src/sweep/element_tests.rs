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
//! Beacon and probe-response elements: both walkers find what the building
//! says, and a lie stops the walk where it is, never past the frame. A
//! lying length or a truncated last element hides itself and what follows,
//! and every honest element before it is still found.

use super::check::agree;
use super::model::build;
use super::rng::{Rng, SEEDS};

const ROUNDS: usize = 3000;

#[test]
fn sound_elements_are_all_found() {
    for seed in SEEDS {
        let mut rng = Rng::new(seed);
        for _ in 0..ROUNDS {
            let b = build(&mut rng);
            agree(&b.tags, |id| b.first(id, b.at.len()).map(<[u8]>::to_vec));
        }
    }
}

#[test]
fn a_lying_length_hides_itself_and_what_follows_and_nothing_before() {
    for seed in SEEDS {
        let mut rng = Rng::new(seed);
        for _ in 0..ROUNDS {
            let mut b = build(&mut rng);
            let k = rng.below(b.at.len());
            let start = b.at[k].1;
            let room = b.tags.len() - start - 2;
            if room >= 255 {
                continue;
            }
            b.tags[start + 1] = (room + 1 + rng.below(255 - room)) as u8;
            agree(&b.tags, |id| b.first(id, k).map(<[u8]>::to_vec));
        }
    }
}

#[test]
fn a_truncated_last_element_hides_only_itself() {
    for seed in SEEDS {
        let mut rng = Rng::new(seed);
        for _ in 0..ROUNDS {
            let mut b = build(&mut rng);
            let last = b.at.len() - 1;
            let start = b.at[last].1;
            let cut = start + 1 + rng.below(b.tags.len() - start - 1);
            b.tags.truncate(cut);
            agree(&b.tags, |id| b.first(id, last).map(<[u8]>::to_vec));
        }
    }
}
