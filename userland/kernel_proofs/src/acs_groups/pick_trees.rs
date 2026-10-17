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

//! Whole seeded trees for the pick sweeps, and the buses below a bridge.

use super::pick_gen::{Tree, ROOT};
use super::port_rule::UPSTREAM;

/// `on_root`: some switches sit straight on the root bus, their upstream port
/// saying root or nothing (L77e).
pub(super) fn grown(seed: u64, on_root: bool) -> Tree {
    let mut t = Tree(Vec::new(), Vec::new(), 0, seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    for _ in 0..1 + t.draw(4) {
        let said = [None, Some(ROOT), Some(ROOT)][t.draw(3) as usize];
        let kind = if on_root && t.draw(2) == 0 { UPSTREAM } else { ROOT };
        t.bridge(0, kind, said, usize::MAX, 0);
    }
    t
}

/// The last bus below `at`: the greatest any bridge in its subtree names.
pub(super) fn last_below(t: &Tree, at: usize) -> u8 {
    let mut last = t.0[at].1.unwrap_or(0);
    for d in 0..t.0.len() {
        let mut u = d;
        while u != at && u != usize::MAX {
            u = t.1[u].1;
        }
        if u == at {
            last = last.max(t.0[d].1.unwrap_or(0));
        }
    }
    last
}
