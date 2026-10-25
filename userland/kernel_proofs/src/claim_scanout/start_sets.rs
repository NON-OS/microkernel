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

//! The starts the takeover model's searches run over (L79).

use super::starts::{Cfg, Sib};

/// Every mix with G1 alone, bootfb claiming once or twice and the kernel
/// trying two draws; with G2 beside it, bootfb claiming once and one draw,
/// or claiming twice with no sibling, no exit and one draw.
pub(super) fn every() -> Vec<Cfg> {
    let mut all = Vec::new();
    for bits in 0..16u8 {
        let [g2, doomed, again, exit] = [0, 1, 2, 3].map(|i| bits & 1 << i != 0);
        for sib in [Sib::Idle, Sib::Unmap, Sib::Stray] {
            let wide = g2 && again && (exit || sib != Sib::Idle);
            if !wide {
                all.push(Cfg { g2, doomed, again, sib, exit, draws: 2 - u8::from(g2) });
            }
        }
    }
    all
}

/// Those of `every` that `keep` keeps.
pub(super) fn some(keep: impl Fn(&Cfg) -> bool) -> Vec<Cfg> {
    every().into_iter().filter(keep).collect()
}
