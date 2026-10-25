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
//! Element lists built sound, with where each one sits known from the
//! building, not from any walk: the soundness the sweeps check the crate's
//! walkers against.

use super::rng::Rng;

/// Ids the station reads (SSID, DS, RSN, MDE, Extended Capabilities,
/// vendor, RSNXE) and some it passes over.
pub const IDS: [u8; 10] = [0, 3, 48, 54, 127, 221, 244, 1, 50, 255];

pub struct Built {
    pub tags: Vec<u8>,
    /// Each element's id, and where it starts in `tags`.
    pub at: Vec<(u8, usize)>,
}

/// One to twelve elements, each with a sound length.
pub fn build(rng: &mut Rng) -> Built {
    let (mut tags, mut at) = (Vec::new(), Vec::new());
    for _ in 0..1 + rng.below(12) {
        let id = if rng.below(4) == 0 { rng.byte() } else { rng.pick(&IDS) };
        let len = if rng.below(8) == 0 { rng.below(256) } else { rng.below(40) };
        at.push((id, tags.len()));
        tags.push(id);
        tags.push(len as u8);
        tags.extend(rng.bytes(len));
    }
    Built { tags, at }
}

impl Built {
    /// The first element with `id` among the first `honest`, whole.
    pub fn first(&self, id: u8, honest: usize) -> Option<&[u8]> {
        let (_, start) = *self.at[..honest].iter().find(|e| e.0 == id)?;
        let len = usize::from(self.tags[start + 1]);
        Some(&self.tags[start..start + 2 + len])
    }
}
