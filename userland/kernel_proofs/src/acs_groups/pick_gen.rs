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

//! Seeded bridge trees for the pick sweep (L77): root ports, switches nested
//! three deep, ports reporting anything, bridges firmware left unset.

use super::port_rule::{Bridge, DOWNSTREAM, ROOT_PORT as ROOT, UPSTREAM};

/// The bridges, each one's true kind and the index of the bridge above it.
pub(super) struct Tree(
    pub(super) Vec<Bridge>,
    pub(super) Vec<(u8, usize)>,
    pub(super) u8,
    pub(super) u64,
);

impl Tree {
    pub(super) fn draw(&mut self, n: u64) -> u64 {
        self.3 ^= self.3 << 13;
        self.3 ^= self.3 >> 7;
        self.3 ^= self.3 << 17;
        self.3 % n
    }
    fn told(&mut self, honest: u8) -> Option<u8> {
        let lies = [Some(honest), Some(honest), Some(UPSTREAM), Some(ROOT), Some(DOWNSTREAM), None];
        lies[self.draw(6) as usize]
    }
    pub(super) fn bridge(&mut self, bus: u8, kind: u8, said: Option<u8>, up: usize, depth: u32) {
        let (me, below) = (self.0.len(), self.2 + 1);
        self.2 = below;
        self.0.push((bus, Some(below), said));
        self.1.push((kind, up));
        if self.draw(4) == 0 {
            let (unset, said) =
                ([Some(0), None, Some(below)][self.draw(3) as usize], self.told(ROOT));
            self.0.push((below, unset, said));
            self.1.push((0, me));
        } else if depth < 3 && self.draw(2) == 0 {
            let said = self.told(UPSTREAM);
            self.bridge(below, UPSTREAM, said, me, depth + 1);
        }
        for _ in 0..if kind == UPSTREAM { 1 + self.draw(4) } else { 0 } {
            let said = self.told(DOWNSTREAM);
            self.bridge(below, DOWNSTREAM, said, me, depth + 1);
        }
    }
}
