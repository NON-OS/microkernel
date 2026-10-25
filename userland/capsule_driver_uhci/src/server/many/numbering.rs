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
//! What the front needs of one controller.
//! Host port numbers: each controller's root ports numbered on after the
//! controllers before it.

use super::front::Front;
use super::hc::Hc;

impl<H: Hc> Front<H> {
    /// Host port `port` as (controller, its own port).
    pub fn local(&self, port: u8) -> Option<(usize, u8)> {
        let mut base = 0u8;
        for (k, hc) in self.hcs.iter().enumerate() {
            let Some(hc) = hc else { continue };
            if (base + 1..=base.saturating_add(hc.ports())).contains(&port) {
                return Some((k, port - base));
            }
            base = base.saturating_add(hc.ports());
        }
        None
    }

    /// Controller `k`'s first host port, less one.
    pub fn base(&self, k: usize) -> u8 {
        self.hcs[..k].iter().flatten().fold(0u8, |b, h| b.saturating_add(h.ports()))
    }
}
